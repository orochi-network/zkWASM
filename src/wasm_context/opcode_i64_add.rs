use crate::memory::memory::Memory;
use crate::opcode::wasm_opcode::WasmOpcode;
use crate::proof_context::proof_context::ProofContext;
use crate::wasm_context::wasm_context::WasmContext;

impl WasmContext {
    pub(crate) fn execute_opcode_i64_add(
        &mut self,
        proof_context: &mut ProofContext,
        pc_before_executing: &u64,
        iaddr_before_executing: &u64,
        stack_depth_before_executing: &usize,
    ) -> WasmOpcode {
        let b = self.stack_pop();

        let a = self.stack_pop();
        // Let's consider this is a cheat
        // we put the result to memory, to check the ability to write
        // Don't expect the Wasm runtime have the same behaviour
        let (program_memory_start, _) = self.get_mut_memory().get_section_size_from_section_index(2);

        self.get_mut_memory()
            .write(&(a + b).to_be_bytes(), program_memory_start, 8);

        println!("{}|{}\tadd\t\t{} {}", self.get_pc().clone(), self.get_iaddr().clone(), a, b);

        self.inc_iaddr(1);
        // Increase program counter
        self.inc_pc();


        // collect trace
        // proof_context.collect_trace_opcode_i64_add(&mut self.time_stamp, self.pc, self.iaddr);
        // TODO: fix the line above

        WasmOpcode::I64Add(a, b)
    }
}