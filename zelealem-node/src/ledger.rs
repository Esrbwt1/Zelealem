// Import the new types from our crypto module.
use crate::crypto::{Hash, PublicKey, Signature};

// State Objects (SOs) are the fundamental components of the ledger.
pub struct StateObject {
    pub id: Hash, // The SO is identified by the hash of its contents.
    pub owner: PublicKey, // The public key of the owner.
    pub data: Vec<u8>,
    pub validation_logic: Vec<u8>,
}

// A Causal Link allows one transaction to reference the logic of another State Object.
pub struct CausalLink {
    pub source_so_id: Hash,
    pub target_so_id: Hash,
}

// A transaction consumes and creates State Objects.
pub struct Transaction {
    pub id: Hash, // The transaction is identified by the hash of its contents.
    pub inputs: Vec<Hash>, // A list of SO IDs that this transaction consumes.
    pub outputs: Vec<StateObject>,
    pub causal_links: Vec<CausalLink>,
    pub signature: Signature, // A quantum-resistant signature from the owner of the inputs.
}

// A Block is a collection of transactions.
pub struct Block {
    pub id: Hash, // The block is identified by the hash of its contents.
    pub proposer: PublicKey,
    pub transactions: Vec<Transaction>,
    pub vdf_proof: Vec<u8>, // The Verifiable Delay Function output.
}