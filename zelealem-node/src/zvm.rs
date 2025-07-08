use crate::bytecode::OpCode;

// The Zelealem Virtual Machine
pub struct ZVM {
    /// The smart contract code to execute.
    bytecode: Vec<u8>,
    /// The execution stack. We will start by only allowing 64-bit signed integers.
    stack: Vec<i64>,
    /// The Program Counter, pointing to the next instruction to be executed.
    pc: usize,
}

impl ZVM {
    /// Creates a new ZVM instance ready to run the given bytecode.
    pub fn new(bytecode: Vec<u8>) -> Self {
        Self {
            bytecode,
            stack: Vec::new(),
            pc: 0,
        }
    }

    /// The main execution loop of the ZVM.
    /// This is the "Fetch-Decode-Execute" cycle.
    pub fn run(&mut self) -> Result<i64, String> {
        loop {
            // 1. Fetch
            let opcode_byte = self.bytecode.get(self.pc).cloned().ok_or("PC out of bounds")?;
            self.pc += 1;

            // 2. Decode
            let opcode = OpCode::from(opcode_byte);

            // 3. Execute
            match opcode {
                OpCode::Halt => {
                    // Stop execution and return the top value of the stack.
                    return self.stack.pop().ok_or("Execution halted on empty stack".to_string());
                }
                OpCode::Push => {
                    // The PUSH opcode is followed by 8 bytes representing the i64 value.
                    let value_bytes: [u8; 8] = self.bytecode[self.pc..self.pc + 8]
                        .try_into()
                        .map_err(|e| format!("Failed to read push argument: {}", e))?;
                    let value = i64::from_le_bytes(value_bytes);
                    self.stack.push(value);
                    self.pc += 8; // Advance PC past the 8-byte argument.
                }
                OpCode::Add => {
                    let b = self.stack.pop().ok_or("ADD requires two values on the stack")?;
                    let a = self.stack.pop().ok_or("ADD requires two values on the stack")?;
                    self.stack.push(a + b);
                }
                OpCode::Sub => {
                    let b = self.stack.pop().ok_or("SUB requires two values on the stack")?;
                    let a = self.stack.pop().ok_or("SUB requires two values on the stack")?;
                    self.stack.push(a - b);
                }
            }
        }
    }
}