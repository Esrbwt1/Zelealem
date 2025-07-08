use crate::ledger::Transaction;
use std::collections::VecDeque;

const MAX_MEMPOOL_SIZE: usize = 1000;

// The Mempool holds transactions that have been received but not yet
// included in a block. A VecDeque is a double-ended queue.
#[derive(Debug)]
pub struct Mempool {
    transactions: VecDeque<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: VecDeque::new(),
        }
    }

    /// Adds a transaction to the mempool if there is space.
    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        if self.transactions.len() >= MAX_MEMPOOL_SIZE {
            println!("Mempool is full. Rejecting transaction.");
            return false;
        }
        self.transactions.push_back(tx);
        true
    }

    /// Retrieves a batch of transactions to be included in a new block.
    pub fn get_batch(&mut self, max_txs: usize) -> Vec<Transaction> {
        let batch_size = self.transactions.len().min(max_txs);
        self.transactions.drain(0..batch_size).collect()
    }
}