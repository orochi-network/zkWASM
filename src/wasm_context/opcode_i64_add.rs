use crate::memory::memory::Memory;
use crate::opcode::wasm_opcode::WasmOpcode;
use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::r#type::section_type::SectionType;
use crate::wasm_context::wasm_context::WasmContext;

impl WasmContext {
    pub(crate) fn execute_opcode_i64_add(
        &mut self,
        proof_context: &mut ProofContext,
        pc_before_executing: &u64,
        iaddr_before_executing: &u64,
        stack_depth_before_executing: &usize,
        byte_code: &u16,
    ) -> WasmOpcode {
        let b_location = self.get_stack().len() as u64 - 1;
        let b = self.stack_pop();

        let a_location = self.get_stack().len() as u64 - 1;
        let a = self.stack_pop();
        // Let's consider this is a cheat
        // we put the result to memory, to check the ability to write
        // Don't expect the Wasm runtime have the same behaviour
        let (program_memory_start, _) = self.get_mut_memory().get_section_size_from_section_index(2);

        let addition_result_in_bytes = &(a + b).to_be_bytes();
        let addition_result_section_types: [SectionType; 8] = (0..8).into_iter().map(|i| {
            SectionType::from_memory_section(
                &self.get_mut_memory().get_section_from_offset(
                    program_memory_start + i
                )
            )
        }).collect::<Vec<SectionType>>().try_into().unwrap();

        self.get_mut_memory()
            .write(addition_result_in_bytes, program_memory_start, 8);

        println!("{}|{}\tadd\t\t{} {}", self.get_pc().clone(), self.get_iaddr().clone(), a, b);

        self.inc_iaddr(1);
        // Increase program counter
        self.inc_pc();


        // collect trace
        proof_context.collect_trace_opcode_i64_add(
            pc_before_executing.clone(),
            iaddr_before_executing.clone(),
            stack_depth_before_executing.clone(),
            byte_code.clone(),
            b_location,
            b,
            a_location,
            a,
            &addition_result_section_types,
            program_memory_start,
            addition_result_in_bytes,
        );

        WasmOpcode::I64Add(a, b)
    }
}