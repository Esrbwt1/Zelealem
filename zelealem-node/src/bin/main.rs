// We need to use the zelealem_node library we've built.
use zelealem_node::node::Node;

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

    // 1. Create a new Node. This also initializes the network swarm.
    let mut node = Node::new().await;

    // 2. Set up network listening addresses.
    // Listen on all interfaces on a specific port.
    // The OS will assign a specific IP address (e.g., 192.168.1.10).
    node.swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .unwrap();

    // Create a specific topic for our blockchain data.
    let topic = gossipsub::IdentTopic::new("zelealem-blocks");
    node.swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap();


    println!("Node initialized. Listening for connections...");

    // 3. The Main Event Loop
    // This loop is the heart of the node's execution. It continuously
    // checks for events from the network swarm and acts on them.
    loop {
        // `select!` is a Tokio macro that waits on multiple async operations
        // and executes the block for the one that completes first.
        select! {
            event = node.swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Node listening on: {}", address);
                    }
                    SwarmEvent::Behaviour(p2p_event) => {
                        // Handle events from our custom ZelealemBehaviour
                        match p2p_event {
                            zelealem_node::p2p::ZelealemBehaviourEvent::Mdns(mdns_event) => {
                                match mdns_event {
                                    mdns::Event::Discovered(list) => {
                                        for (peer_id, multiaddr) in list {
                                            println!("mDNS discovered a new peer: {}", peer_id);
                                            node.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                        }
                                    },
                                    mdns::Event::Expired(list) => {
                                        for (peer_id, multiaddr) in list {
                                            println!("mDNS peer has expired: {}", peer_id);
                                            node.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                                        }
                                    }
                                }
                            }
                            zelealem_node::p2p::ZelealemBehaviourEvent::Gossipsub(gossip_event) => {
                                // For now, we just print gossip events.
                                // In the future, this is where we would receive new blocks and transactions.
                                println!("Received gossipsub event: {:?}", gossip_event);
                            }
                        }
                    }
                    _ => {
                        // Handle other swarm events (connections opened, closed, etc.)
                        // println!("Unhandled swarm event: {:?}", event);
                    }
                }
            }
        }
    }
}