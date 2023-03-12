use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::state_trace_tuple::StateTraceTuple;

impl ProofContext {
    pub fn plainly_check_opcode_i64_add(
        &self,
        current_state_trace_tuple: &StateTraceTuple,
        next_state_trace_tuple: &StateTraceTuple,
    ) {

    }

    fn check_iaddr_and_pc_consistent(
        current_state_trace_tuple: &StateTraceTuple,
        next_state_trace_tuple: &StateTraceTuple,
    ) {
        assert_eq!(
            current_state_trace_tuple.get_iaddr_before_executing() + 1, // since we need to keep param_index
            next_state_trace_tuple.get_iaddr_before_executing()
        );
        assert_eq!(
            current_state_trace_tuple.get_pc_before_executing() + 1,
            next_state_trace_tuple.get_pc_before_executing()
        );
    }
}