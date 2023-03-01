use crate::memory::memory::{Memory, MemorySection, WasmMemory};
use crate::opcode::wasm_opcode::WasmOpcode;
use crate::util::constant_setting::NUM_BYTES_FOR_LOCAL_GET;

pub struct WasmContext {
    pc: u64,
    iaddr: u64,
    memory: WasmMemory,
    stack: Vec<u64>,
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
        }
    }

    pub(crate) fn get_pc(& mut self) -> &u64 {
        &mut self.pc
    }

    pub(crate) fn get_iaddr(&mut self) -> &u64 {
        &mut self.iaddr
    }

    pub(crate) fn get_mut_memory<'a>(&'a mut self) -> &'a mut WasmMemory {
        &mut self.memory
    }

    pub(crate) fn inc_pc(&mut self) {
        self.pc += 1;
    }

    pub(crate) fn inc_iaddr(&mut self, opcode_size: usize) {
        self.iaddr += opcode_size as u64;
    }

    pub(crate) fn stack_push(&mut self, val: u64) {
        self.stack.push(val);
    }

    pub(crate) fn stack_pop(&mut self) -> u64 {
        self.stack.pop().expect("Stack is empty")
    }

    pub(crate) fn get_stack<'a>(&'a self) -> &'a Vec<u64> {
        &self.stack
    }
}

