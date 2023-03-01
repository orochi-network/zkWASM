use crate::wasm_context::wasm_context::WasmContext;
use crate::memory::memory::{Memory, MemorySection, WasmMemory};
use crate::opcode::wasm_opcode::WasmOpcode;
use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::section_type::SectionType;
use crate::util::constant_setting::NUM_BYTES_FOR_LOCAL_GET;

impl WasmContext {

    pub fn next(&mut self, proof_context: &mut ProofContext) -> WasmOpcode {
        let (
            pc_before_executing,
            iaddr_before_executing,
            stack_depth_before_executing
        ) = (
            self.get_pc().clone(),
            self.get_iaddr().clone(),
            self.get_stack().len().clone()
        );
        // Read byte code at current iaddr
        let byte_code = self.get_mut_memory().read(iaddr_before_executing, 1)[0] as u16;

        // Matching byte_code to Wasm opcode
        match byte_code {
            0x0b => self.execute_opcode_end(
                proof_context, &pc_before_executing, &iaddr_before_executing,
                &stack_depth_before_executing, &byte_code,
            ),
            0x20 => self.execute_opcode_i64_local_get(
                proof_context, &pc_before_executing, &iaddr_before_executing,
                &stack_depth_before_executing, &byte_code,
            ),
            0x7c => self.execute_opcode_i64_add(
                proof_context, &pc_before_executing, &iaddr_before_executing,
                &stack_depth_before_executing, &byte_code,
            ),
            _ => WasmOpcode::Unreachable,
        }
    }
}