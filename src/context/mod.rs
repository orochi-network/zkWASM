use crate::{
    memory::{Memory, MemorySection, WasmMemory},
    opcode::WasmOpcode,
};
use crate::trace::state_trace_manager::StateTraceManager;
use crate::trace::storage_access_record::{AccessType, SectionType, StorageAccessRecord, StorageType};
use crate::trace::state_trace_tuple::StateTraceTuple;
use crate::trace_collector::opcode_0x0b::Collector0x0b;

pub struct WasmContext {
    pc: u64,
    iaddr: u64,
    memory: WasmMemory,
    stack: Vec<u64>,
    time_stamp: u64,

    // this part is for constructing proof
    state_trace_manager: StateTraceManager,
}

impl WasmContext {
    pub fn new(code_image: &[u8], initial_memory: &[u8]) -> Self {
        let mut memory = WasmMemory::new(1024);
        let (code_image_len, initial_memory_len) =
            (code_image.len() as u64, initial_memory.len() as u64);
        let executable_image_len = code_image_len + initial_memory_len;

        // Init executable image
        memory.init(code_image, 0, code_image_len);
        memory.init(initial_memory, code_image_len, initial_memory_len);
        memory.add_section(MemorySection::ExecutableImage(
            (0, code_image_len),
            (code_image_len, executable_image_len),
        ));
        memory.add_section(MemorySection::ProgramMemory((executable_image_len, 1024)));
        Self {
            pc: 1,
            iaddr: 0, // Point to the code image
            memory,
            stack: Vec::new(),
            time_stamp: 0,

            state_trace_manager: StateTraceManager::new(),
        }
    }

    pub fn pc(&self) -> u64 {
        self.pc
    }

    pub fn iaddr(&self) -> u64 {
        self.iaddr
    }

    pub fn memory(&'static self) -> &'static WasmMemory {
        &self.memory
    }

    pub fn inc_pc(&mut self) {
        self.pc += 1;
    }

    pub fn inc_iaddr(&mut self, opcode_size: usize) {
        self.iaddr += opcode_size as u64;
    }

    pub fn push(&mut self, val: u64) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> u64 {
        self.stack.pop().expect("Stack is empty")
    }

    pub fn next(&mut self) -> WasmOpcode {
        // Read byte code at current iaddr
        let byte_code = self.memory.read(self.iaddr(), 1)[0] as u16;

        // Matching byte_code to Wasm opcode
        match byte_code {
            0x0b => {
                println!("{}|{}\tend", self.pc, self.iaddr);
                self.inc_iaddr(1);
                self.inc_pc();

                // collect trace
                self.state_trace_manager.collect0x0b(&mut self.time_stamp, self.pc, self.iaddr);
                WasmOpcode::End
            }
            0x20 => {
                // Seek iaddr to param index
                self.inc_iaddr(1);
                let param_index = self.memory.read(self.iaddr, 1)[0] as u8;
                // Read data from param index to stack
                // Section 1 is initial memory, i'm to lazy to create a constant
                // We don't have the WASI https://wasi.dev/ that's why I implement in this way.
                // I put the param in initial memory
                let (param_start, _) = self.memory.get_section(1);

                // i64 is 8 bytes
                let param = u64::from_be_bytes(
                    self.memory
                        .read(param_start + (param_index as u64 * 8), 8)
                        .try_into()
                        .unwrap(),
                );

                println!(
                    "{}|{}\tlocal.get\t{} i64 {}",
                    self.pc,
                    self.iaddr - 1,
                    param_index,
                    param
                );
                // Put param to stack
                self.stack.push(param);
                // Skip param index byte
                self.inc_iaddr(1);
                // Increase program counter
                self.inc_pc();

                // collect trace

                // 0xfe is i64
                WasmOpcode::LocalGet(param_index, 0xfe)
            }
            0x7c => {
                let (b, a) = (
                    self.stack.pop().expect("Stack is empty"),
                    self.stack.pop().expect("Stack is empty"),
                );
                // Let's consider this is a cheat
                // we put the result to memory, to check the ability to write
                // Don't expect the Wasm runtime have the same behaviour
                let (program_memory_start, _) = self.memory.get_section(2);

                self.memory
                    .write(&(a + b).to_be_bytes(), program_memory_start, 8);

                println!("{}|{}\tadd\t\t{} {}", self.pc, self.iaddr, a, b);

                self.inc_iaddr(1);
                // Increase program counter
                self.inc_pc();


                // collect trace
                WasmOpcode::I64Add(a, b)
            }
            _ => WasmOpcode::Unreachable,
        }
    }
}
