use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::proof_type::proof_opcode::ProofOpcode;
use crate::proof_context::trace::state_trace_tuple::StateTraceTuple;

impl ProofContext {
    pub fn verify_single_trace_in_plain(
        current_state_trace_tuple: &StateTraceTuple,
        next_state_trace_tuple: &StateTraceTuple,
    ) -> bool {
        for read_location in current_state_trace_tuple.get_read_locations() {
            
        }
        match current_state_trace_tuple.get_proof_opcode() {
            ProofOpcode::Unreachable => {
                todo!()
            },
            ProofOpcode::End => {
                todo!()
            },
            ProofOpcode::LocalGet => {
                todo!()
            },
            ProofOpcode::I64Add => {
                todo!()
            },
        }
    }
}