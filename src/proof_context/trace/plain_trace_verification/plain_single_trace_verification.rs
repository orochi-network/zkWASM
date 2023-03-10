use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::proof_type::proof_opcode::ProofOpcode;
use crate::proof_context::trace::state_trace_tuple::StateTraceTuple;

impl ProofContext {
    fn verify_single_trace_in_plain(
        current_state_trace_tuple: &StateTraceTuple,
        next_state_trace_tuple: &StateTraceTuple,
    ) {
        // check correct increase in time tag
        let current_ram_access_locations = current_state_trace_tuple.get_ram_access_locations();
        for index in 0..current_ram_access_locations.len() - 1 {
            assert_eq!(
                current_ram_access_locations[index].get_time_stamp() + 1,
                current_ram_access_locations[index + 1].get_time_stamp()
            )
        }

        let last_index = current_ram_access_locations.len() - 1;
        assert_eq!(
            current_ram_access_locations[last_index].get_time_stamp() + 1,
            next_state_trace_tuple.get_ram_access_locations()[0].get_time_stamp()
        );
        
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

    pub fn verify_trace_in_plain(&self) {
        // check consistency in proof_context trace
        let state_trace_manager = self.get_state_trace_manager();
        for index in 0..state_trace_manager.size() - 1 {
            Self::verify_single_trace_in_plain(
                state_trace_manager.get_state_trace_tupe(index),
                state_trace_manager.get_state_trace_tupe(index + 1)
            );
        }
    }
}