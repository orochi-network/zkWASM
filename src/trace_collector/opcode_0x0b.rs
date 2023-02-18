use crate::trace::state_trace_manager::StateTraceManager;
use crate::trace::state_trace_tuple::{MAX_NUM_READ_LOCATIONS, MAX_NUM_WRITE_LOCATIONS, StateTraceTuple};
use crate::trace::storage_access_record::{AccessType, SectionType, StorageAccessRecord, StorageType};

pub trait Collector0x0b {
    fn collect0x0b(&mut self, time_stamp: &mut u64, pc: u64, iaddr: u64);
}

impl Collector0x0b for StateTraceManager {
    fn collect0x0b(&mut self, time_stamp: &mut u64, pc: u64, iaddr: u64) {
        self.add_state_trace_tuple(
            StateTraceTuple::new(
                pc,
                iaddr,
                [
                    StorageAccessRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        time_stamp.clone(),
                        AccessType::Read,
                    ),
                    StorageAccessRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        time_stamp.clone() + 1,
                        AccessType::Read,
                    ),
                    StorageAccessRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        time_stamp.clone() + 2,
                        AccessType::Read,
                    ),
                ],
                [
                    StorageAccessRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        time_stamp.clone() + 3,
                        AccessType::Write,
                    ),
                    StorageAccessRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        time_stamp.clone() + 4,
                        AccessType::Write,
                    ),
                    StorageAccessRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        time_stamp.clone() + 5,
                        AccessType::Write,
                    ),
                ]
            )
        );
        *time_stamp += MAX_NUM_READ_LOCATIONS as u64;
        *time_stamp += MAX_NUM_WRITE_LOCATIONS as u64;
    }
}