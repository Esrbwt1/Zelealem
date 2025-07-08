use zelealem_node::{
    bytecode::OpCode,
    zvm::ZVM,
};

#[test]
fn test_simple_arithmetic() {
    // This test will execute the program `(10 + 5) - 3`, which should result in 12.

    // === 1. Assemble the bytecode program ===
    // Our program needs to perform the following steps:
    // PUSH 10  (Push the first number)
    // PUSH 5   (Push the second number)
    // ADD      (Add them, stack is now [15])
    // PUSH 3   (Push the third number)
    // SUB      (Subtract, stack is now [12])
    // HALT     (Stop and return the result)

    let mut bytecode = Vec::new();

    // PUSH 10
    bytecode.push(OpCode::Push as u8);
    bytecode.extend_from_slice(&10i64.to_le_bytes());

    // PUSH 5
    bytecode.push(OpCode::Push as u8);
    bytecode.extend_from_slice(&5i64.to_le_bytes());

    // ADD
    bytecode.push(OpCode::Add as u8);

    // PUSH 3
    bytecode.push(OpCode::Push as u8);
    bytecode.extend_from_slice(&3i64.to_le_bytes());
    
    // SUB
    bytecode.push(OpCode::Sub as u8);

    // HALT
    bytecode.push(OpCode::Halt as u8);

    // === 2. Execute the program with the ZVM ===
    let mut vm = ZVM::new(bytecode);
    let result = vm.run().expect("ZVM execution failed");

    // === 3. Assert the result ===
    // The final value on the stack should be 12.
    assert_eq!(result, 12);
    println!("SUCCESS: ZVM correctly calculated (10 + 5) - 3 = 12");
}

#[test]
fn test_stack_underflow_error() {
    // This test ensures our ZVM correctly fails when an operation
    // doesn't have enough arguments on the stack.

    // Program:
    // PUSH 10
    // ADD   <- This should fail because ADD needs two values.
    // HALT
    let bytecode = vec![
        OpCode::Push as u8,
        10i64.to_le_bytes()[0], 10i64.to_le_bytes()[1], 10i64.to_le_bytes()[2], 10i64.to_le_bytes()[3],
        10i64.to_le_bytes()[4], 10i64.to_le_bytes()[5], 10i64.to_le_bytes()[6], 10i64.to_le_bytes()[7],
        OpCode::Add as u8,
        OpCode::Halt as u8,
    ];

    let mut vm = ZVM::new(bytecode);
    let result = vm.run(); // We expect this to return an Err.

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "ADD requires two values on the stack");
    println!("SUCCESS: ZVM correctly panicked on stack underflow.");
}