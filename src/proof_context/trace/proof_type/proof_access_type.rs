#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ProofAccessType {
    Read = 0,
    Write = 1,
}