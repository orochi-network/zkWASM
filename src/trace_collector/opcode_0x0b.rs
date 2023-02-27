use crate::trace::section_type::SectionType;
use crate::trace::state_trace_manager::StateTraceManager;
use crate::trace::state_trace_tuple;
use crate::trace::state_trace_tuple::{StateTraceTuple};
use crate::trace::storage_read_record::{StorageReadRecord};
use crate::trace::storage_type::StorageType;
use crate::trace::storage_write_record::StorageWriteRecord;
use crate::util::{util, constant_setting::ConstantValue};

impl StateTraceManager {
    pub fn collect_0x0b(
        &mut self,
        time_stamp_before_executing: &mut u64,
        pc_before_executing: u64,
        iaddr_before_executing: u64,
        stack_depth_before_executing: usize,
    ) {
        let read_locations: [StorageReadRecord; state_trace_tuple::MAX_NUM_READ_LOCATIONS] =
            (0..state_trace_tuple::MAX_NUM_READ_LOCATIONS).into_iter().map(|_|
                StorageReadRecord::new(
                    StorageType::Undefined,
                    SectionType::Undefined,
                    0,
                    0,
                    util::get_value_and_increase::<u64>(time_stamp_before_executing),
                )
            ).collect::<Vec<StorageReadRecord>>().try_into().unwrap();

        let write_locations: [StorageWriteRecord; state_trace_tuple::MAX_NUM_WRITE_LOCATIONS] =
            (0..state_trace_tuple::MAX_NUM_WRITE_LOCATIONS).into_iter().map(|_|
                StorageWriteRecord::new(
                    StorageType::Undefined,
                    SectionType::Undefined,
                    0,
                    0,
                    util::get_value_and_increase::<u64>(time_stamp_before_executing),
                )
            ).collect::<Vec<StorageWriteRecord>>().try_into().unwrap();

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