use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::state_trace_tuple::StateTraceTuple;

impl ProofContext {
    pub fn verify_single_trace_in_plain(
        current_state_trace_tuple: &StateTraceTuple,
        next_state_trace_tuple: &StateTraceTuple,
    ) -> bool {
        todo!()
    }
}