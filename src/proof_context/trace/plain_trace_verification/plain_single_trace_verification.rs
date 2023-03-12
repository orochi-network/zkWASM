use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::proof_type::proof_opcode::ProofOpcode;
use crate::proof_context::trace::state_trace_tuple::StateTraceTuple;

impl ProofContext {
    fn verify_single_trace_in_plain(
        &self,
        current_state_trace_tuple: &StateTraceTuple,
        next_state_trace_tuple: &StateTraceTuple,
    ) {
        // check correct increase in time tag
        let current_ram_access_records = current_state_trace_tuple.get_ram_access_records();
        for index in 0..current_ram_access_records.len() - 1 {
            assert_eq!(
                current_ram_access_records[index].get_time_stamp() + 1,
                current_ram_access_records[index + 1].get_time_stamp()
            );
        }

        let last_index = current_ram_access_records.len() - 1;
        assert_eq!(
            current_ram_access_records[last_index].get_time_stamp() + 1,
            next_state_trace_tuple.get_ram_access_records()[0].get_time_stamp()
        );
        match current_state_trace_tuple.get_proof_opcode() {
            ProofOpcode::Unreachable => {
                todo!()
            },
            ProofOpcode::End => {
                todo!()
            },
            ProofOpcode::LocalGet => {
                self.plainly_check_opcode_local_get(
                    current_state_trace_tuple,
                    next_state_trace_tuple,
                );
            },
            ProofOpcode::I64Add => {
                self.plainly_check_opcode_i64_add(
                    current_state_trace_tuple,
                    next_state_trace_tuple,
                );
            },
        };
    }

    pub fn verify_trace_in_plain(&self) {
        // check consistency in proof_context trace
        let state_trace_manager = self.get_state_trace_manager();
        for index in 0..state_trace_manager.size() - 1 {
            print!(
                "Checking step {index} with opcode {:?}: ",
                state_trace_manager.get_state_trace_tuple(index).get_proof_opcode()
            );
            self.verify_single_trace_in_plain(
                state_trace_manager.get_state_trace_tuple(index),
                state_trace_manager.get_state_trace_tuple(index + 1)
            );
            println!(" passed!");
        }
    }

    pub(crate) fn verify_in_range(min_value: u64, max_value: u64, value_to_check: u64) {
        assert!((min_value <= value_to_check) && (value_to_check <= max_value));
    }
}