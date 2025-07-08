use crate::crypto::{self, Hash};
use crate::ledger::Transaction;
use crate::state_db::{StateDB, StateError};
use serde::Serialize;
use thiserror::Error;

// A comprehensive list of every reason a transaction might be invalid.
#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("Transaction ID hash does not match its content")]
    MismatchedId,
    #[error("The cryptographic signature is invalid")]
    InvalidSignature,
    #[error("An input State Object with ID {0:?} was not found")]
    InputNotFound(Hash),
    #[error("Transaction has no inputs and therefore no authority to act")]
    NoInputs,
    #[error("Inputs are not all owned by the same public key")]
    MultipleOwners,
    #[error("Internal state database error: {0}")]
    StateError(#[from] StateError), // Allows automatic conversion from a StateError
}

// The TransactionValidator holds a reference to the current state.
// It uses this state to validate new transactions.
pub struct TransactionValidator<'a> {
    state_db: &'a StateDB,
}

// This temporary struct is re-defined here to avoid making the one in `ledger.rs` public.
// It's a helper for re-calculating the transaction's hash for validation.
#[derive(Serialize)]
struct HashableTransaction<'a> {
    inputs: &'a Vec<Hash>,
    outputs: &'a Vec<crate::ledger::StateObject>,
    causal_links: &'a Vec<crate::ledger::CausalLink>,
}

impl<'a> TransactionValidator<'a> {
    pub fn new(state_db: &'a StateDB) -> Self {
        Self { state_db }
    }

    /// Validates a transaction against the current state.
    /// This is the master function that performs all checks in order.
    pub fn validate_transaction(&self, tx: &Transaction) -> Result<(), ValidationError> {
        self.check_id_hash(tx)?;
        self.check_inputs_exist(tx)?;
        self.check_signature(tx)?;
        Ok(())
    }

    /// Check 1: Verifies that the transaction's `id` field is the correct
    /// hash of its contents. This prevents tampering.
    fn check_id_hash(&self, tx: &Transaction) -> Result<(), ValidationError> {
        let hashable_part = HashableTransaction {
            inputs: &tx.inputs,
            outputs: &tx.outputs,
            causal_links: &tx.causal_links,
        };
        let bytes =
            bincode::serde::encode_to_vec(&hashable_part, bincode::config::standard())
                .unwrap(); // Should not fail if serialization in `new` succeeded.
        let expected_hash = crypto::hash_data(&bytes);

        if tx.id != expected_hash {
            return Err(ValidationError::MismatchedId);
        }
        Ok(())
    }

    /// Check 2: Ensures that every input State Object referenced by the transaction
    /// actually exists in our current state database.
    fn check_inputs_exist(&self, tx: &Transaction) -> Result<(), ValidationError> {
        if tx.inputs.is_empty() {
            return Err(ValidationError::NoInputs);
        }
        for input_id in &tx.inputs {
            self.state_db.get_so(input_id)?;
        }
        Ok(())
    }

    /// Check 3: Verifies the cryptographic signature.
    /// This proves that the rightful owner of the input assets authorized this transaction.
    fn check_signature(&self, tx: &Transaction) -> Result<(), ValidationError> {
        // Rule: A transaction must be signed by the owner of its inputs.
        // We retrieve the public key of the owner of the *first* input.
        let first_input_id = tx.inputs.get(0).ok_or(ValidationError::NoInputs)?;
        let first_input_so = self.state_db.get_so(first_input_id)?;
        let owner_pub_key = first_input_so.owner;

        // Now, verify all other inputs are owned by the same key.
        // This prevents creating a transaction that spends assets from multiple people.
        for input_id in &tx.inputs[1..] {
            let so = self.state_db.get_so(input_id)?;
            if so.owner != owner_pub_key {
                return Err(ValidationError::MultipleOwners);
            }
        }
        
        // Verify the signature against the transaction's ID hash using the owner's public key.
        if !crypto::verify_signature(&tx.signature, &tx.id, &owner_pub_key) {
            return Err(ValidationError::InvalidSignature);
        }

        Ok(())
    }
}