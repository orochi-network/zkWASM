use crate::trace::section_type::SectionType;
use crate::trace::state_trace_manager::StateTraceManager;
use crate::trace::state_trace_tuple::{MAX_NUM_READ_LOCATIONS, StateTraceTuple};
use crate::trace::storage_read_record::{StorageReadRecord};
use crate::trace::storage_type::StorageType;
use crate::trace::storage_write_record::StorageWriteRecord;
use crate::util::{
    util,
    constant_setting
};
use crate::util::constant_setting::NUM_BYTES_FOR_LOCAL_GET;

impl StateTraceManager {
    pub fn collect_0x20(
        &mut self,
        time_stamp_before_executing: &mut u64,
        pc_before_executing: u64,
        iaddr_before_executing: u64,
        stack_depth_before_executing: usize,
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
                    util::get_value_and_increase::<u64>(time_stamp_before_executing)
                )
            );

            for i in 0..NUM_BYTES_FOR_LOCAL_GET {
                res.push(
                    StorageReadRecord::new(
                        StorageType::Memory,
                        section_types_of_read_locations[i].clone(),
                        first_index_read + i as u64,
                        read_bytes[i] as u64,
                        util::get_value_and_increase::<u64>(time_stamp_before_executing)
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
                    util::get_value_and_increase::<u64>(time_stamp_before_executing)
                )
            );

            res.push(
                StorageWriteRecord::new(
                    StorageType::Undefined,
                    SectionType::Undefined,
                    0,
                    0,
                    util::get_value_and_increase::<u64>(time_stamp_before_executing),
                )
            );

            res.try_into().unwrap()
        };

        self.add_state_trace_tuple(
            StateTraceTuple::new(
                pc_before_executing,
                iaddr_before_executing,
                stack_depth_before_executing,
                read_locations,
                write_locations,
            )
        );
    }
}