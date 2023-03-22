/// We used u64 to store everything regarding for trivial machine
/// Every machine have 64 bits in word size except Wasm v128 and EVM 256 bits
pub struct UntypedValue {
    bits: u64,
}

/// In WebAssembly there are some certain way to interactive with the memory
/// Following this `enum` it defined three possible actions
pub enum MemoryAction {
    Init,
    Read,
    Write,
}

/// I/O trace for the memory
pub struct MemoryTrace {
    time_log: u64,
    action: MemoryAction,
    address: &UntypedValue,
    value: UntypedValue,
}

/// Raw memory just use for the storage
/// I thinking about time series data for parallelisation
#[derive(Debug, Clone)]
pub struct MemoryRaw {
    ptr: &UntypedValue,
    memory_raw: Vec<u8>,
    memory_trace: Vec<MemoryTrace>,
}

/// zkMemory
pub struct Memory {
    raw: memory_raw,
    commitment: Vec<MemoryCommitment>,
}

/// Have no idea about memory commitment but it will be great if we use verkle tree commitment
/// To prove the memory
struct MemoryCommitment {}

/// The interface for zkMemory
trait MemoryInterface {
    fn new(raw_memory: MemoryRaw) -> Self;
    fn init(&mut self, address: u64) -> Result<MemoryCommitment, Error>;
    fn read(&mut self, address: u64) -> Result<MemoryCommitment, Error>;
    fn write(&mut self, address: u64, chunk: u64) -> Result<MemoryCommitment, Error>;
}

impl MemoryInterface for Memory {}
