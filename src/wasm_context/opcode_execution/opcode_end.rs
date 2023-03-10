use crate::wasm_context::wasm_context::WasmContext;
use crate::opcode::wasm_opcode::WasmOpcode;
use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::proof_type::proof_opcode::ProofOpcode;

impl WasmContext {
    pub(crate) fn execute_opcode_end(
        &mut self,
        proof_context: &mut ProofContext,
        pc_before_executing: u64,
        iaddr_before_executing: u64,
        stack_depth_before_executing: usize,
    ) -> (WasmOpcode, ProofOpcode) {
        println!("{}|{}\tend", self.get_pc().clone(), self.get_iaddr().clone());
        self.inc_iaddr(1);
        self.inc_pc();
        // collect trace
        let proof_opcode = proof_context.collect_trace_opcode_end(
            pc_before_executing,
            iaddr_before_executing,
            stack_depth_before_executing,
        );
        (WasmOpcode::End, proof_opcode)
    }
}