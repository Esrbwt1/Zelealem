// We need to tell Rust that we are using the zelealem_node library in this test.
use zelealem_node::{
    crypto::{self, sign_data},
    ledger::{StateObject, Transaction},
    state_db::StateDB,
    validator::TransactionValidator,
};

// #[test] is an attribute that tells Rust this function is a test.
#[test]
fn test_valid_transaction_flow() {
    // === 1. SETUP ===
    // Create an empty state database for our world.
    let mut state = StateDB::new();

    // Create a user, "Alice", by generating a keypair.
    let (alice_pub_key, alice_sec_key) = crypto::generate_keypair();

    // Create an initial asset (a State Object) and give it to Alice.
    // This is like mining a coin or receiving a deposit.
    let initial_so = StateObject::new(
        alice_pub_key,      // Owned by Alice
        vec![100],          // Some data representing, e.g., 100 tokens
        vec![],             // No special validation logic for now
    );
    let initial_so_id = initial_so.id;

    // Add this new asset to the world's state.
    state.add_so(initial_so).unwrap();

    // === 2. TRANSACTION CREATION ===
    // Alice wants to spend her asset. She creates a new transaction.
    // This transaction will consume her initial SO...
    let mut tx = Transaction::new(
        vec![initial_so_id], // Input: The asset she's spending
        vec![],              // Outputs: No new assets created for this simple case
        vec![],              // Causal Links: None
    );

    // Alice signs the transaction with her secret key to authorize it.
    // The signature is over the transaction's ID (its hash).
    let signature = sign_data(&tx.id, &alice_sec_key);
    tx.sign(signature);

    // === 3. VALIDATION ===
    // A node on the network receives this transaction and must validate it.
    // It creates a validator that has access to the current world state.
    let validator = TransactionValidator::new(&state);

    // We assert that the validation succeeds.
    // `is_ok()` will be true if the result is Ok(()), and will cause the test
    // to fail if it's an Err(ValidationError).
    assert!(validator.validate_transaction(&tx).is_ok());
    println!("SUCCESS: Valid transaction was correctly approved.");
}

#[test]
fn test_invalid_signature_flow() {
    // === 1. SETUP ===
    let mut state = StateDB::new();
    let (alice_pub_key, _alice_sec_key) = crypto::generate_keypair(); // We don't need Alice's key

    // Create a malicious user, "Bob".
    let (_bob_pub_key, bob_sec_key) = crypto::generate_keypair();

    let initial_so = StateObject::new(alice_pub_key, vec![100], vec![]);
    let initial_so_id = initial_so.id;
    state.add_so(initial_so).unwrap();

    // === 2. TRANSACTION CREATION ===
    // Bob creates a transaction trying to spend ALICE's asset.
    let mut tx = Transaction::new(vec![initial_so_id], vec![], vec![]);

    // Bob signs the transaction with HIS secret key.
    let bob_signature = sign_data(&tx.id, &bob_sec_key);
    tx.sign(bob_signature);

    // === 3. VALIDATION ===
    // A node validates the transaction.
    let validator = TransactionValidator::new(&state);

    // We assert that the validation FAILS.
    // `is_err()` will be true if the result is an error.
    let result = validator.validate_transaction(&tx);
    assert!(result.is_err());

    // We can even check for the specific error type.
    assert_eq!(
        result.unwrap_err(),
        zelealem_node::validator::ValidationError::InvalidSignature
    );
    println!("SUCCESS: Transaction with invalid signature was correctly rejected.");
}