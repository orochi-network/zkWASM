use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::section_type::SectionType;
use crate::proof_context::trace::state_trace_manager::StateTraceManager;
use crate::proof_context::trace::state_trace_tuple;
use crate::proof_context::trace::state_trace_tuple::StateTraceTuple;
use crate::proof_context::trace::storage_read_record::StorageReadRecord;
use crate::proof_context::trace::storage_type::StorageType;
use crate::proof_context::trace::storage_write_record::StorageWriteRecord;

impl ProofContext {
    pub fn collect_trace_opcode_end(
        &mut self,
        pc_before_executing: &u64,
        iaddr_before_executing: &u64,
        stack_depth_before_executing: &usize,
        byte_code: &u16,
    ) {
        let read_locations: [StorageReadRecord; state_trace_tuple::MAX_NUM_READ_LOCATIONS] =
            (0..state_trace_tuple::MAX_NUM_READ_LOCATIONS).into_iter().map(|_|
                StorageReadRecord::new(
                    StorageType::Undefined,
                    SectionType::Undefined,
                    0,
                    0,
                    self.get_time_stamp_then_increase(),
                )
            ).collect::<Vec<StorageReadRecord>>().try_into().unwrap();

        let write_locations: [StorageWriteRecord; state_trace_tuple::MAX_NUM_WRITE_LOCATIONS] =
            (0..state_trace_tuple::MAX_NUM_WRITE_LOCATIONS).into_iter().map(|_|
                StorageWriteRecord::new(
                    StorageType::Undefined,
                    SectionType::Undefined,
                    0,
                    0,
                    self.get_time_stamp_then_increase(),
                )
            ).collect::<Vec<StorageWriteRecord>>().try_into().unwrap();

        self.add_state_trace_tuple(
            &StateTraceTuple::new(
                pc_before_executing.clone(),
                iaddr_before_executing.clone(),
                stack_depth_before_executing.clone(),
                byte_code.clone(),
                read_locations,
                write_locations,
            )
        );
    }
}