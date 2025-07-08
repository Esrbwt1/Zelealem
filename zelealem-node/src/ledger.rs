// Represents a unique identifier for a State Object or a Transaction.
// For now, it's a simple number, but will be replaced by a cryptographic hash.
pub type Identifier = u64;

// State Objects (SOs) are the fundamental components of the ledger, analogous to UTXOs.
// They contain data and the logic that governs their consumption.
pub struct StateObject {
    pub id: Identifier,
    pub owner: String, // Will be replaced by a public key (quantum-resistant signature).
    pub data: Vec<u8>, // Arbitrary data stored in the SO.
    pub validation_logic: Vec<u8>, // The script/logic that must be satisfied to consume this SO.
}

// A Causal Link allows one transaction to reference the logic of another State Object.
// This is the key to enabling contract-to-contract calls in our model.
pub struct CausalLink {
    pub source_so_id: Identifier, // The SO providing the logic.
    pub target_so_id: Identifier, // The SO being acted upon.
}

// A transaction consumes a set of existing State Objects and creates a set of new ones.
// It is the atomic unit of change in the Zelealem network.
pub struct Transaction {
    pub id: Identifier,
    pub inputs: Vec<Identifier>, // A list of SO IDs that this transaction consumes.
    pub outputs: Vec<StateObject>, // A list of new SOs created by this transaction.
    pub causal_links: Vec<CausalLink>, // Links for cross-contract logic.
    pub signature: Vec<u8>, // A quantum-resistant signature from the owner of the inputs.
}

// A Block is a collection of transactions, validated and added to the blockchain.
pub struct Block {
    pub id: Identifier,
    pub proposer: String, // Public key of the block proposer.
    pub transactions: Vec<Transaction>,
    pub vdf_proof: Vec<u8>, // The Verifiable Delay Function output (proof of time).
}