use libp2p::gossipsub;

// The topic for gossiping new, unconfirmed transactions.
pub const TRANSACTIONS_TOPIC: &str = "transactions";
// The topic for gossiping newly minted blocks.
pub const BLOCKS_TOPIC: &str = "blocks";

pub fn transactions_topic() -> gossipsub::IdentTopic {
    gossipsub::IdentTopic::new(TRANSACTIONS_TOPIC)
}

pub fn blocks_topic() -> gossipsub::IdentTopic {
    gossipsub::IdentTopic::new(BLOCKS_TOPIC)
}