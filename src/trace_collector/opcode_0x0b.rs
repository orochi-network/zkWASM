use crate::trace::state_trace_manager::StateTraceManager;
use crate::trace::state_trace_tuple;
use crate::trace::state_trace_tuple::{StateTraceTuple};
use crate::trace::storage_access_record::{
    SectionType, StorageReadRecord, StorageWriteRecord, StorageType
};
use crate::util::{util, constant_setting::ConstantValue};

impl StateTraceManager {
    pub fn collect0x0b(&mut self, time_stamp: &mut u64, pc: u64, iaddr: u64) {
        let read_locations = {
            let mut res: Vec<StorageReadRecord>= vec![];
            for _ in 0..state_trace_tuple::MAX_NUM_READ_LOCATIONS {
                res.append(
                    &mut StorageReadRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        util::get_value_and_increase::<u64>(time_stamp),
                    )
                );
            }
            res
        };

        let write_locations = {
            let &mut res: Vec<StorageWriteRecord>= vec![];
            for _ in 0..state_trace_tuple::MAX_NUM_WRITE_LOCATIONS {
                res.append(
                    &mut StorageWriteRecord::new(
                        StorageType::Undefined,
                        SectionType::Undefined,
                        0,
                        0,
                        util::get_value_and_increase::<u64>(time_stamp),
                    ));
            }
            res
        };

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