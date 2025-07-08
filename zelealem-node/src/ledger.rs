use crate::crypto::{Hash, PublicKey, Signature};
use serde::Serialize;
use serde_big_array::BigArray;
use serde::Deserialize;

// This is the correct function to use when using serde::Serialize with bincode 2.x
use bincode::serde::encode_to_vec;
use bincode::config::standard;


// At the top of ledger.rs
#[derive(Serialize)]
struct HashableBlock<'a> {
    proposer: &'a PublicKey,
    transactions: &'a Vec<Transaction>,
    vdf_proof: &'a Vec<u8>,
    // IMPORTANT: A block's hash must also depend on the previous block's hash.
    previous_hash: &'a Hash,
}

// A temporary struct used only for the purpose of hashing a State Object.
#[derive(Serialize)]
struct HashableStateObject<'a> {
    owner: &'a PublicKey,
    data: &'a Vec<u8>,
    validation_logic: &'a Vec<u8>,
}

// A temporary struct used for hashing the core content of a Transaction.
#[derive(Serialize)]
struct HashableTransaction<'a> {
    inputs: &'a Vec<Hash>,
    outputs: &'a Vec<StateObject>,
    causal_links: &'a Vec<CausalLink>,
}

// State Objects (SOs) are the fundamental components of the ledger.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StateObject {
    pub id: Hash,
    pub owner: PublicKey,
    pub data: Vec<u8>,
    pub validation_logic: Vec<u8>,
}

impl StateObject {
    pub fn new(owner: PublicKey, data: Vec<u8>, validation_logic: Vec<u8>) -> Self {
        let hashable_part = HashableStateObject {
            owner: &owner,
            data: &data,
            validation_logic: &validation_logic,
        };
        // THE CORRECT API CALL
        let bytes = encode_to_vec(&hashable_part, standard()).expect("Failed to serialize SO");
        let id = crate::crypto::hash_data(&bytes);

        Self {
            id,
            owner,
            data,
            validation_logic,
        }
    }
}

// A Causal Link allows one transaction to reference the logic of another State Object.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CausalLink {
    pub source_so_id: Hash,
    pub target_so_id: Hash,
}

// A transaction consumes and creates State Objects.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub id: Hash,
    pub inputs: Vec<Hash>,
    pub outputs: Vec<StateObject>,
    pub causal_links: Vec<CausalLink>,
    #[serde(with = "BigArray")]
    pub signature: Signature,
}

impl Transaction {
    pub fn new(
        inputs: Vec<Hash>,
        outputs: Vec<StateObject>,
        causal_links: Vec<CausalLink>,
    ) -> Self {
        let hashable_part = HashableTransaction {
            inputs: &inputs,
            outputs: &outputs,
            causal_links: &causal_links,
        };
        // THE CORRECT API CALL
        let bytes = encode_to_vec(&hashable_part, standard()).expect("Failed to serialize TX");
        let id = crate::crypto::hash_data(&bytes);

        Self {
            id,
            inputs,
            outputs,
            causal_links,
            signature: [0u8; 64],
        }
    }

    pub fn sign(&mut self, signature: Signature) {
        self.signature = signature;
    }
}

// A Block is a collection of transactions.
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub id: Hash,
    pub previous_hash: Hash, // Link to the previous block
    pub proposer: PublicKey,
    pub transactions: Vec<Transaction>,
    pub vdf_proof: Vec<u8>,
}

impl Block {
    // Constructor for a new Block.
    pub fn new(
        previous_hash: Hash,
        proposer: PublicKey,
        transactions: Vec<Transaction>,
        vdf_proof: Vec<u8>,
    ) -> Self {
        let hashable_part = HashableBlock {
            previous_hash: &previous_hash,
            proposer: &proposer,
            transactions: &transactions,
            vdf_proof: &vdf_proof,
        };
        let bytes =
            bincode::serde::encode_to_vec(&hashable_part, bincode::config::standard())
                .expect("Failed to serialize Block");
        let id = crate::crypto::hash_data(&bytes);

        Self {
            id,
            previous_hash,
            proposer,
            transactions,
            vdf_proof,
        }
    }
}