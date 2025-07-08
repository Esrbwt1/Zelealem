// #[repr(u8)] tells the Rust compiler to represent this enum as a single, unsigned 8-bit integer (a byte).
// This is crucial because our bytecode will be a stream of bytes.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    /// 0x00: Halts execution.
    Halt = 0x00,
    /// 0x01: Pushes the next 8 bytes in the bytecode onto the stack as an i64 integer.
    Push = 0x01,
    /// 0x02: Pops two values from the stack, adds them, and pushes the result back.
    Add = 0x02,
    /// 0x03: Pops two values from the stack, subtracts the top from the second-to-top, and pushes the result.
    Sub = 0x03,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => OpCode::Halt,
            0x01 => OpCode::Push,
            0x02 => OpCode::Add,
            0x03 => OpCode::Sub,
            _ => panic!("Invalid opcode: {}", byte),
        }
    }
}