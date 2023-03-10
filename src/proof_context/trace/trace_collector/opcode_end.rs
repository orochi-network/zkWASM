use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::proof_type::proof_opcode::ProofOpcode;
use crate::proof_context::trace::state_trace_tuple;
use crate::proof_context::trace::state_trace_tuple::StateTraceTuple;
use crate::proof_context::trace::ram_access_record::RamAccessRecord;

impl ProofContext {
    pub fn collect_trace_opcode_end(
        &mut self,
        pc_before_executing: &u64,
        iaddr_before_executing: &u64,
        stack_depth_before_executing: &usize,
    ) -> ProofOpcode {
        let ram_access_locations: [RamAccessRecord; state_trace_tuple::MAX_NUM_RAM_ACCESS_LOCATIONS] =
            (0..state_trace_tuple::MAX_NUM_RAM_ACCESS_LOCATIONS).into_iter().map(|_|
                RamAccessRecord::dummy(self.get_time_stamp_then_increase())
            ).collect::<Vec<RamAccessRecord>>().try_into().unwrap();

        let proof_opcode = ProofOpcode::End;

        self.add_state_trace_tuple(
            &StateTraceTuple::new(
                pc_before_executing.clone(),
                iaddr_before_executing.clone(),
                stack_depth_before_executing.clone(),
                proof_opcode.clone(),
                ram_access_locations,
            )
        );

        proof_opcode
    }
}