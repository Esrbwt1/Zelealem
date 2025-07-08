use std::collections::HashMap;
use crate::ledger::StateObject;
use crate::crypto::Hash;
use thiserror::Error;

// Define custom errors for our database operations for clearer error handling.
#[derive(Error, Debug, PartialEq)]
pub enum StateError {
    #[error("State Object with ID {0:?} already exists")]
    AlreadyExists(Hash),
    #[error("State Object with ID {0:?} not found")]
    NotFound(Hash),
}

// StateDB is our in-memory key-value store for State Objects.
// The key is the StateObject's unique Hash (ID), and the value is the SO itself.
#[derive(Default)]
pub struct StateDB {
    objects: HashMap<Hash, StateObject>,
}

impl StateDB {
    // Creates a new, empty state database.
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    // Adds a State Object to the database.
    // Returns an error if an object with the same ID already exists.
    pub fn add_so(&mut self, so: StateObject) -> Result<(), StateError> {
        if self.objects.contains_key(&so.id) {
            return Err(StateError::AlreadyExists(so.id));
        }
        self.objects.insert(so.id, so);
        Ok(())
    }

    // Retrieves a reference to a State Object from the database.
    // Returns an error if the object is not found.
    pub fn get_so(&self, id: &Hash) -> Result<&StateObject, StateError> {
        self.objects.get(id).ok_or(StateError::NotFound(*id))
    }

    // Removes a State Object from the database, consuming it.
    // Returns the removed object or an error if it was not found.
    pub fn remove_so(&mut self, id: &Hash) -> Result<StateObject, StateError> {
        self.objects.remove(id).ok_or(StateError::NotFound(*id))
    }
}