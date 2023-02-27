#[derive(Debug)]
pub enum WasmOpcode {
    Unreachable,
    // Depend on function definition we have different kind of data
    // https://webassembly.github.io/spec/core/appendix/index-types.html
    LocalGet(u8, u8),
    // Add two i64
    I64Add(u64, u64),
    // End function write data to program's memory
    // This is an example so we have many hard code
    End,
}

impl WasmOpcode {
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Unreachable => 0x00,
            Self::LocalGet(_, _) => 0x20,
            Self::I64Add(_, _) => 0x7c,
            Self::End => 0x0b,
        }
    }
}
