#[derive(Clone)]
pub enum ProofOpcode {
    Unreachable = 0x00,
    End = 0x0b,
    LocalGet = 0x20,
    I64Add = 0x7c,
}

impl ProofOpcode {
    pub fn to_u16(&self) -> u16 {
        match self {
            Self::Unreachable => 0x00,
            Self::End => 0x0b,
            Self::LocalGet => 0x20,
            Self::I64Add=> 0x7c,
        }
    }
}