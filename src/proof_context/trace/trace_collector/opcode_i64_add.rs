use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::r#type::section_type::SectionType;
use crate::proof_context::trace::r#type::storage_type::StorageType;
use crate::proof_context::trace::state_trace_manager::StateTraceManager;
use crate::proof_context::trace::state_trace_tuple::{MAX_NUM_READ_LOCATIONS, StateTraceTuple};
use crate::proof_context::trace::storage_read_record::StorageReadRecord;
use crate::proof_context::trace::storage_write_record::StorageWriteRecord;

impl ProofContext {
    pub fn collect_trace_opcode_i64_add(
        &mut self,
        pc_before_executing: u64,
        iaddr_before_executing: u64,
        stack_depth_before_executing: usize,
        byte_code: u16,
        b_location: u64, b: u64,
        a_location: u64, a: u64,
        addition_result_in_section_types: &[SectionType; 8],
        addition_result_starting_location: u64,
        addition_result_in_bytes: &[u8; 8],
    ) {
        let read_locations: [StorageReadRecord; MAX_NUM_READ_LOCATIONS] = {
            let mut res = Vec::<StorageReadRecord>::new();
            res.push(
                StorageReadRecord::new(
                    StorageType::Stack,
                    SectionType::Stack,
                    b_location,
                    b,
                    self.get_time_stamp_then_increase()
                )
            );

            res.push(
                StorageReadRecord::new(
                    StorageType::Stack,
                    SectionType::Stack,
                    a_location,
                    a,
                    self.get_time_stamp_then_increase()
                )
            );

            for _ in 2..MAX_NUM_READ_LOCATIONS {
                res.push(StorageReadRecord::dummy(self.get_time_stamp_then_increase()));
            }

            res.try_into().unwrap()
        };
        let write_locations = {
            let mut res = Vec::<StorageWriteRecord>::new();
            for i in 0..8 {
                res.push(
                    StorageWriteRecord::new(
                        StorageType::Memory,
                        addition_result_in_section_types[i].clone(),
                        addition_result_starting_location + i as u64,
                        addition_result_in_bytes[i] as u64,
                        self.get_time_stamp_then_increase(),
                    )
                );
            }

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