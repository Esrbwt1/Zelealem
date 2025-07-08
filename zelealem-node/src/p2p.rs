use libp2p::{gossipsub, mdns, ping, swarm::NetworkBehaviour}; // Add ping

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "ZelealemBehaviourEvent")] // We need to specify the event type
pub struct ZelealemBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub ping: ping::Behaviour, // Add the ping protocol
}

// Define the custom event our behaviour can emit
#[derive(Debug)]
pub enum ZelealemBehaviourEvent {
    Mdns(mdns::Event),
    Gossipsub(gossipsub::Event),
    Ping(ping::Event),
}

impl From<mdns::Event> for ZelealemBehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        ZelealemBehaviourEvent::Mdns(event)
    }
}

impl From<gossipsub::Event> for ZelealemBehaviourEvent {
    fn from(event: gossipsub::Event) -> Self {
        ZelealemBehaviourEvent::Gossipsub(event)
    }
}

impl From<ping::Event> for ZelealemBehaviourEvent {
    fn from(event: ping::Event) -> Self {
        ZelealemBehaviourEvent::Ping(event)
    }
}