use crate::crypto::{PublicKey, Hash};
use std::collections::HashMap;

// The amount of currency staked. For now, a simple number.
pub type Stake = u64;

// A Validator is a participant who has staked assets to secure the network.
#[derive(Clone, Debug)]
pub struct Validator {
    pub pub_key: PublicKey,
    pub stake: Stake,
}

// The ValidatorSet manages all active validators.
#[derive(Clone, Debug)]
pub struct ValidatorSet {
    // A map from the validator's public key to their validator info.
    pub validators: HashMap<PublicKey, Validator>,
}

impl ValidatorSet {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
        }
    }

    pub fn add_validator(&mut self, validator: Validator) {
        self.validators.insert(validator.pub_key, validator);
    }

    /// Selects a block proposer for a given round.
    /// This is a simplified round-robin selection based on the hash of the
    /// previous block, making it deterministic. A real PoS system would
    /// use a more complex, stake-weighted random algorithm.
    pub fn select_proposer(&self, previous_block_hash: Hash) -> Option<PublicKey> {
        if self.validators.is_empty() {
            return None;
        }

        // "Sort" the validators to get a deterministic order.
        let mut sorted_keys: Vec<PublicKey> = self.validators.keys().cloned().collect();
        sorted_keys.sort();

        // Use the previous block hash to deterministically pick an index.
        let mut seed = [0u8; 8];
        seed.copy_from_slice(&previous_block_hash[..8]);
        let round_number = u64::from_le_bytes(seed);
        
        let index = round_number as usize % sorted_keys.len();
        
        sorted_keys.get(index).cloned()
    }
}