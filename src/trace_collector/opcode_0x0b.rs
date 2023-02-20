use crate::trace::state_trace_manager::StateTraceManager;
use crate::trace::state_trace_tuple;
use crate::trace::state_trace_tuple::{StateTraceTuple};
use crate::trace::storage_access_record::{AccessType, SectionType, StorageAccessRecord, StorageType};
use crate::util::{util, constant_setting::ConstantValue};

impl StateTraceManager {
    pub fn collect0x0b(&mut self, time_stamp: &mut u64, pc: u64, iaddr: u64) {
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
                        util::get_value_and_increase::<u64>(time_stamp),
                        AccessType::Read,
                    ),
                    StorageAccessRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        util::get_value_and_increase::<u64>(time_stamp),
                        AccessType::Read,
                    ),
                    StorageAccessRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        util::get_value_and_increase::<u64>(time_stamp),
                        AccessType::Read,
                    ),
                ],
                [
                    StorageAccessRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        util::get_value_and_increase::<u64>(time_stamp),
                        AccessType::Write,
                    ),
                    StorageAccessRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        util::get_value_and_increase::<u64>(time_stamp),
                        AccessType::Write,
                    ),
                    StorageAccessRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        util::get_value_and_increase::<u64>(time_stamp),
                        AccessType::Write,
                    ),
                ]
            )
        );
    }
}