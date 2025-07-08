use zelealem_node::{ledger::Transaction, topics, crypto};
use libp2p::{
    gossipsub, identity, noise, ping,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, SwarmBuilder,
};
use tokio::time::{timeout, Duration};
use libp2p::futures::StreamExt; // CORRECTED: Import the necessary trait

#[tokio::main]
async fn main() {
    // === 1. SETUP: A behaviour with gossipsub AND ping ===
    let id_keys = identity::Keypair::generate_ed25519();
    
    #[derive(NetworkBehaviour)]
    #[behaviour(to_swarm = "WalletBehaviourEvent")]
    struct WalletBehaviour {
        gossipsub: gossipsub::Behaviour,
        ping: ping::Behaviour,
    }

    #[derive(Debug)]
    enum WalletBehaviourEvent {
        Gossipsub(gossipsub::Event),
        Ping(ping::Event),
    }
    impl From<gossipsub::Event> for WalletBehaviourEvent { fn from(v: gossipsub::Event) -> Self { Self::Gossipsub(v) } }
    impl From<ping::Event> for WalletBehaviourEvent { fn from(v: ping::Event) -> Self { Self::Ping(v) } }

    let behaviour = WalletBehaviour {
        gossipsub: gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(id_keys.clone()),
            gossipsub::Config::default(),
        ).unwrap(),
        ping: ping::Behaviour::new(ping::Config::new()),
    };
    
    let mut swarm = SwarmBuilder::with_existing_identity(id_keys)
        .with_tokio()
        .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)
        .unwrap()
        .with_behaviour(|_| behaviour)
        .unwrap()
        .build();

    let tx_topic = topics::transactions_topic();
    swarm.behaviour_mut().gossipsub.subscribe(&tx_topic).unwrap();

    // === 2. DIAL AND WAIT FOR A PONG ===
    let target_node_addr: Multiaddr = "/ip4/127.0.0.1/tcp/52585".parse().expect("Failed to parse address"); // <-- IMPORTANT: CHANGE THIS PORT
    swarm.dial(target_node_addr.clone()).unwrap();
    println!("Wallet client started. Dialing node and waiting for a pong...");

    // Event loop to wait for a successful ping
    if let Ok(_) = timeout(Duration::from_secs(10), async {
        loop {
            // CORRECTED: The path to a successful ping event is simpler now.
            if let SwarmEvent::Behaviour(WalletBehaviourEvent::Ping(ping::Event { result: Ok(_), .. })) = swarm.select_next_some().await {
                println!("Successfully pinged the node!");
                return;
            }
        }
    }).await {
        println!("Ping successful. Publishing transaction...");

        // === 3. CREATE AND PUBLISH TRANSACTION ===
        let (_tx_pub_key, tx_sec_key) = crypto::generate_keypair();
        let mut tx = Transaction::new(vec![[0; 32]], vec![], vec![]);
        let signature = crypto::sign_data(&tx.id, &tx_sec_key);
        tx.sign(signature);
        
        let serialized_tx = bincode::serde::encode_to_vec(&tx, bincode::config::standard()).unwrap();
        if let Err(e) = swarm.behaviour_mut().gossipsub.publish(tx_topic, serialized_tx) {
            println!("Error publishing transaction: {:?}", e);
        } else {
            println!("Successfully published dummy transaction!");
        }

    } else {
        println!("Failed to ping the node within 10 seconds.");
    }

    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Wallet client exiting.");
}