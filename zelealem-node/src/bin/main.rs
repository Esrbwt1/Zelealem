// We need to use the zelealem_node library we've built.
use zelealem_node::node::Node;
use zelealem_node::consensus::Validator;
use zelealem_node::ledger::Block;
use zelealem_node::ledger::Transaction; 
use zelealem_node::topics; // New
use zelealem_node::validator::TransactionValidator; // New
use std::time::Duration;
use tokio::time::interval;

// Import libp2p components needed for the main loop.
use libp2p::{
    gossipsub,
    mdns,
    swarm::{SwarmEvent},
};
use tokio::select;
use libp2p::futures::StreamExt;
// The `main` function of our executable.
// It must be marked `#[tokio::main]` to run within the Tokio async runtime.
#[tokio::main]
async fn main() {
    println!("Zelealem Node - Initializing...");

    let mut node = Node::new().await;

    // --- Manually set up a validator for testing ---
    // In a real system, this would come from staking transactions.
    // For now, we'll make our own node a validator.
    // To get our specific PublicKey type, we must hash the public part of the id_keys.
    // Get the PeerId from the node's swarm, which is the canonical public identity.
    let local_peer_id = *node.swarm.local_peer_id();
    // Convert the PeerId to bytes to create a hashable representation for our consensus.
    let local_pub_key = zelealem_node::crypto::PublicKey(zelealem_node::crypto::hash_data(&local_peer_id.to_bytes()));
    let validator = Validator {
        pub_key: local_pub_key,
        stake: 1000, // Stake 1000 units
    };
    node.validator_set.add_validator(validator);
    println!("Local node registered as a validator.");
    // ---------------------------------------------

    node.swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .unwrap();

    let topic = gossipsub::IdentTopic::new("zelealem-blocks");
    node.swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap();

    let transactions_topic = topics::transactions_topic();
    node.swarm.behaviour_mut().gossipsub.subscribe(&transactions_topic).unwrap();

    // Create a timer that fires every 10 seconds.
    let mut proposer_tick = interval(Duration::from_secs(10));

    println!("Node initialized. Listening for connections and proposing blocks...");

    loop {
        select! {
            // This branch fires every 10 seconds.
            _ = proposer_tick.tick() => {
                println!("\n--- Proposer Tick ---");

                // Check if we are the chosen proposer for the current chain height.
                let latest_hash = node.chain.get_latest_hash();
                if let Some(chosen_proposer) = node.validator_set.select_proposer(latest_hash) {
                    println!("Chosen proposer for this round: {:?}", chosen_proposer);
                                        if chosen_proposer == local_pub_key {
                        println!("It's our turn to propose a block!");
                        
                        // In a real node, we would collect transactions from a mempool.
                        // For now, we create an empty block.
                        // Pull a batch of transactions from the mempool.
                        let transactions = node.mempool.get_batch(10); // Get up to 10 txs
                        if !transactions.is_empty() {
                            println!("Pulled {} transactions from mempool to include in new block.", transactions.len());
                        }

                        let new_block = Block::new(
                            latest_hash,
                            local_pub_key,
                            transactions, // Add the transactions to the block
                            vec![],       // No VDF proof for now
                        );
                        let block_id_for_log = new_block.id; // Clone for logging before move

                        // 1. Process the new block locally.
                        // This updates our own chain and state database.
                        match node.process_block(new_block) {
                            Ok(_) => {
                                println!("Successfully processed our own new block: {:?}", block_id_for_log);
                                
                                // 2. Broadcast the block to the network.
                                // We need to serialize the block to send it.
                                // The block was moved into process_block, so we need to get it back.
                                let last_block = node.chain.get_latest_block().unwrap();
                                let serialized_block = bincode::serde::encode_to_vec(last_block, bincode::config::standard()).unwrap();

                                if let Err(e) = node.swarm.behaviour_mut().gossipsub.publish(topic.clone(), serialized_block) {
                                    println!("Error publishing block: {:?}", e);
                                } else {
                                    println!("Successfully published new block to the network!");
                                }
                            }
                            Err(e) => {
                                // This should not happen if we create the block correctly.
                                println!("Error processing our own block: {:?}", e);
                            }
                        }
                    }
                }
            }

            event = node.swarm.select_next_some() => {
                // (This part is unchanged)
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Node listening on: {}", address);
                    }
                    SwarmEvent::Behaviour(p2p_event) => {
                        match p2p_event {
                            zelealem_node::p2p::ZelealemBehaviourEvent::Mdns(mdns_event) => {
                                match mdns_event {
                                    mdns::Event::Discovered(list) => {
                                        for (peer_id, _multiaddr) in list {
                                            println!("mDNS discovered a new peer: {}", peer_id);
                                            node.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                        }
                                    },
                                    mdns::Event::Expired(list) => {
                                        for (peer_id, _multiaddr) in list {
                                            println!("mDNS peer has expired: {}", peer_id);
                                            node.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                                        }
                                    }
                                }
                            }
                            zelealem_node::p2p::ZelealemBehaviourEvent::Gossipsub(gossip_event) => {
                                if let gossipsub::Event::Message { message, .. } = gossip_event {
                                    // Check which topic the message arrived on.
                                    if message.topic == topics::blocks_topic().hash() {
                                        // TODO: Handle incoming blocks from other nodes
                                        println!("Received new block via gossipsub.");
                                    } else if message.topic == topics::transactions_topic().hash() {
                                        println!("Received new transaction via gossipsub.");
                                        // Try to deserialize the message data into a Transaction.
                                        match bincode::serde::decode_from_slice::<Transaction, _>(&message.data, bincode::config::standard()) {
                                            Ok((tx, _)) => {
                                                println!("Successfully deserialized transaction: {:?}", tx.id);
                                                // Validate the transaction against our current state.
                                                let validator = TransactionValidator::new(&node.state_db);
                                                match validator.validate_transaction(&tx) {
                                                    Ok(_) => {
                                                        println!("Transaction is valid! Adding to mempool.");
                                                        // If valid, add it to our mempool.
                                                        node.mempool.add_transaction(tx);
                                                    }
                                                    Err(e) => {
                                                        println!("Invalid transaction received: {:?}", e);
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                println!("Failed to deserialize transaction: {:?}", e);
                                            }
                                        }
                                    }
                                }
                            }
                            // inside the SwarmEvent::Behaviour match
                            zelealem_node::p2p::ZelealemBehaviourEvent::Ping(event) => {
                                println!("Received ping event: {:?}", event);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}