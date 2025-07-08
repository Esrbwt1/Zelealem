use crate::chain::Chain;
use crate::ledger::Block;
use crate::state_db::StateDB;
use crate::validator::{TransactionValidator, ValidationError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessBlockError {
    #[error("Block's previous_hash does not match the latest block in the chain")]
    MismatchedPreviousHash,
    #[error("Transaction validation failed: {0}")]
    TransactionError(#[from] ValidationError),
}

// The Node represents a single participant in the Zelealem network.
// It owns the canonical state (the chain and the state database).
pub struct Node {
    pub chain: Chain,
    pub state_db: StateDB,
}

impl Node {
    // Creates a brand new node with a genesis block.
    pub fn new() -> Self {
        Self {
            chain: Chain::new(),
            state_db: StateDB::new(),
        }
    }

    /// Processes a new, incoming block.
    /// This is the core function of a node. It validates all transactions
    /// and, if they are all valid, updates the state and the chain atomically.
    pub fn process_block(&mut self, block: Block) -> Result<(), ProcessBlockError> {
        // 1. Basic Block Validation
        let latest_hash = self.chain.get_latest_hash();
        if block.previous_hash != latest_hash {
            return Err(ProcessBlockError::MismatchedPreviousHash);
        }

        // 2. Transaction Validation Loop
        // First, we validate every transaction in the block against the CURRENT state.
        // We do not apply any changes yet. This ensures all transactions are valid
        // based on the state at the beginning of the block.
        let validator = TransactionValidator::new(&self.state_db);
        for tx in &block.transactions {
            validator.validate_transaction(tx)?;
        }

        // 3. State Application Loop
        // If we get here, it means all transactions are valid.
        // Now, and only now, we apply their changes to the state.
        // We can safely borrow mutably now because the validator is no longer in use.
        for tx in &block.transactions {
            // a. Consume inputs
            for input_id in &tx.inputs {
                self.state_db.remove_so(input_id).unwrap(); // Should not fail if validation passed
            }
            // b. Create outputs
            for output_so in &tx.outputs {
                self.state_db.add_so(output_so.clone()).unwrap(); // Should not fail
            }
        }

        // 4. Commit Block
        // The state has been updated. Now add the block to the chain.
        self.chain.add_block(block);

        Ok(())
    }
}