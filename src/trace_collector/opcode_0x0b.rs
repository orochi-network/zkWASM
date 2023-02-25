use crate::trace::state_trace_manager::StateTraceManager;
use crate::trace::state_trace_tuple;
use crate::trace::state_trace_tuple::{StateTraceTuple};
use crate::trace::storage_access_record::{
    SectionType, StorageReadRecord, StorageWriteRecord, StorageType
};
use crate::util::{util, constant_setting::ConstantValue};

impl StateTraceManager {
    pub fn collect_0x0b(&mut self, time_stamp: &mut u64, pc: u64, iaddr: u64) {
        let read_locations: [StorageReadRecord; state_trace_tuple::MAX_NUM_READ_LOCATIONS] =
            (0..state_trace_tuple::MAX_NUM_READ_LOCATIONS).into_iter().map(|_|
                StorageReadRecord::new(
                    StorageType::Undefined,
                    SectionType::Undefined,
                    0,
                    0,
                    util::get_value_and_increase::<u64>(time_stamp),
                )
            ).collect::<Vec<StorageReadRecord>>().try_into().unwrap();

        let write_locations: [StorageWriteRecord; state_trace_tuple::MAX_NUM_WRITE_LOCATIONS] =
            (0..state_trace_tuple::MAX_NUM_WRITE_LOCATIONS).into_iter().map(|_|
                StorageWriteRecord::new(
                    StorageType::Undefined,
                    SectionType::Undefined,
                    0,
                    0,
                    util::get_value_and_increase::<u64>(time_stamp),
                )
            ).collect::<Vec<StorageWriteRecord>>().try_into().unwrap();

        self.add_state_trace_tuple(
            StateTraceTuple::new(
                pc,
                iaddr,
                read_locations,
                write_locations,
            )
        );
    }
}