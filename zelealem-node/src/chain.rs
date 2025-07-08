use crate::ledger::Block;
use crate::crypto::{Hash, PublicKey};

// The blockchain is a sequence of blocks.
#[derive(Default)]
pub struct Chain {
    blocks: Vec<Block>,
}

impl Chain {
    // Creates a new blockchain with a "genesis" block.
    pub fn new() -> Self {
        let genesis_block = Block::new(
        [0u8; 32],          // Previous hash is all zeros
        PublicKey([0u8; 32]), // CORRECTED: Proposer is a null PublicKey struct
        vec![],             // No transactions
        vec![],             // No VDF proof
    );
        Self {
            blocks: vec![genesis_block],
        }
    }

    // Gets the hash of the latest block in the chain.
    pub fn get_latest_hash(&self) -> Hash {
        self.blocks.last().unwrap().id
    }

    // Adds a new block to the chain.
    // NOTE: In a real node, this would involve intense validation.
    // For now, we just add it.
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn get_latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}