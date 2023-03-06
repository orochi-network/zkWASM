use crate::memory::memory::Memory;
use crate::opcode::wasm_opcode::WasmOpcode;
use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::r#type::section_type::SectionType;
use crate::util::constant_setting::NUM_BYTES_FOR_LOCAL_GET;
use crate::wasm_context::wasm_context::WasmContext;

impl WasmContext {
    pub(crate) fn execute_opcode_i64_local_get(
        &mut self,
        proof_context: &mut ProofContext,
        pc_before_executing: &u64,
        iaddr_before_executing: &u64,
        stack_depth_before_executing: &usize,
        byte_code: &u16,
    ) -> WasmOpcode {
        // Seek iaddr to param index
        self.inc_iaddr(1);
        let param_index = {
            let current_iaddr = self.get_iaddr().clone();
            self.get_mut_memory().read(current_iaddr, 1)[0] as u8
        };
        let section_type_of_param_index = {
            let current_iaddr = self.get_iaddr().clone();
            SectionType::from_memory_section(
                &self.get_mut_memory().get_section_from_offset(current_iaddr)
            )
        };
        // Read data from param index to stack
        // Section 1 is initial memory, i'm to lazy to create a constant
        // We don't have the WASI https://wasi.dev/ that's why I implement in this way.
        // I put the param in initial memory
        let (param_start, _) = self.get_mut_memory().get_section_size_from_section_index(1);
        // TODO: change 1 above to constant

        let start_index = param_start + (param_index as u64 * 8);
        let read_bytes: [u8; 8] = self.get_mut_memory()
            .read(start_index, 8)
            .try_into()
            .unwrap();
        // TODO: possibly change 8 above to constant
        let section_types_of_read_locations: [
            SectionType;
            NUM_BYTES_FOR_LOCAL_GET
        ] = (0..NUM_BYTES_FOR_LOCAL_GET).into_iter().map(|i|
            SectionType::from_memory_section(
                &self.get_mut_memory().get_section_from_offset(
                    start_index + i as u64
                )
            )
        ).collect::<Vec<SectionType>>().try_into().unwrap();
        // i64 is 8 bytes
        let param = u64::from_be_bytes(read_bytes);

        println!(
            "{}|{}\tlocal.get\t{} i64 {}",
            self.get_pc().clone(),
            self.get_iaddr(),
            param_index,
            param
        );
        // Put param to stack
        self.stack_push(param);
        // Skip param index byte
        self.inc_iaddr(1);
        // Increase program counter
        self.inc_pc();

        // collect trace
        proof_context.collect_trace_opcode_local_get(
            pc_before_executing.clone(),
            iaddr_before_executing.clone(),
            stack_depth_before_executing.clone(),
            byte_code.clone(),
            section_type_of_param_index,
            param_index as u64,
            &section_types_of_read_locations,
            start_index,
            &read_bytes,
            param,
        );
        // 0xfe is i64
        WasmOpcode::LocalGet(param_index, 0xfe)
    }
}