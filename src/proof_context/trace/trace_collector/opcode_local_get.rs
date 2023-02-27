use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::section_type::SectionType;
use crate::proof_context::trace::state_trace_manager::StateTraceManager;
use crate::proof_context::trace::state_trace_tuple::{MAX_NUM_WRITE_LOCATIONS, StateTraceTuple};
use crate::proof_context::trace::storage_read_record::StorageReadRecord;
use crate::proof_context::trace::storage_type::StorageType;
use crate::proof_context::trace::storage_write_record::StorageWriteRecord;
use crate::util::constant_setting::NUM_BYTES_FOR_LOCAL_GET;

impl ProofContext {
    pub fn collect_trace_opcode_local_get(
        &mut self,
        pc_before_executing: u64,
        iaddr_before_executing: u64,
        stack_depth_before_executing: usize,
        byte_code: u16,
        section_type_of_param_index: SectionType,
        param_index: u64,
        section_types_of_read_locations: &[SectionType; NUM_BYTES_FOR_LOCAL_GET],
        first_index_read: u64,
        read_bytes: &[u8; NUM_BYTES_FOR_LOCAL_GET],
        pushed_stack_value: u64,
    ) {
        let read_locations = {
            let mut res = Vec::<StorageReadRecord>::new();
            res.push(
                StorageReadRecord::new(
                    StorageType::Memory,
                    section_type_of_param_index,
                    iaddr_before_executing as u64,
                    param_index,
                    self.get_time_stamp_then_increase()
                )
            );

            for i in 0..NUM_BYTES_FOR_LOCAL_GET {
                res.push(
                    StorageReadRecord::new(
                        StorageType::Memory,
                        section_types_of_read_locations[i].clone(),
                        first_index_read + i as u64,
                        read_bytes[i] as u64,
                        self.get_time_stamp_then_increase()
                    )
                );
            }

            res.try_into().unwrap()
        };

        let write_locations = {
            let mut res = Vec::<StorageWriteRecord>::new();
            res.push(
                StorageWriteRecord::new(
                    StorageType::Stack,
                    SectionType::Stack,
                    stack_depth_before_executing as u64,
                    pushed_stack_value,
                    self.get_time_stamp_then_increase()
                )
            );

            res.push(
                StorageWriteRecord::new(
                    StorageType::Undefined,
                    SectionType::Undefined,
                    0,
                    0,
                    self.get_time_stamp_then_increase(),
                )
            );

            res.try_into().unwrap()
        };

        self.add_state_trace_tuple(
            &StateTraceTuple::new(
                pc_before_executing,
                iaddr_before_executing,
                stack_depth_before_executing,
                byte_code,
                read_locations,
                write_locations,
            )
        );
    }
}