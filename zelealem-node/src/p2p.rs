use libp2p::{
    gossipsub, mdns, swarm::NetworkBehaviour
};

// This is the core networking logic of a Zelealem node.
// It defines the protocols a node speaks and the events it can produce.
// We are deriving a trait that automatically generates the necessary boilerplate.
#[derive(NetworkBehaviour)]
pub struct ZelealemBehaviour {
    // Gossipsub is the protocol for broadcasting ("gossiping") messages
    // like transactions and new blocks to all interested peers.
    pub gossipsub: gossipsub::Behaviour,

    // mDNS is a local network discovery protocol. It allows nodes on the
    // same local network to find each other without a central server.
    pub mdns: mdns::tokio::Behaviour,
}