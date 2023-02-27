use crate::trace::state_trace_manager::StateTraceManager;
use crate::trace::state_trace_tuple::MAX_NUM_READ_LOCATIONS;
use crate::trace::storage_access_record::{SectionType, StorageReadRecord, StorageType};
use crate::util::{
    util,
    constant_setting
};
use crate::util::constant_setting::MAX_NUM_BYTES_FOR_LOCAL_GET;

impl StateTraceManager {
    pub fn collect_0x20(
        &mut self,
        time_stamp: &mut u64,
        pc: u64,
        iaddr: u64, iaddr_section_type: SectionType, param_index: u64,
        read_first_index: u64, read_values: [u8; MAX_NUM_BYTES_FOR_LOCAL_GET as usize],
    ) {
        let read_locations = {
            let mut res = Vec::<StorageReadRecord>::new();
            res.push(
                StorageReadRecord::new(
                    StorageType::Memory,
                    iaddr_section_type,
                    iaddr,
                    param_index,
                    util::get_value_and_increase::<u64>(time_stamp)
                )
            );

            for i in 0..MAX_NUM_BYTES_FOR_LOCAL_GET {
                res.push(
                    StorageReadRecord::new(
                        StorageType::Memory,
                        SectionType::ProgramMemory
                    )
                );
            }

            res.try_into().unwrap()
        };

            // (0..MAX_NUM_READ_LOCATIONS)
            // .into_iter()
            // .map(
            //     |i|
            //         match i {
            //             0 => { // recording location
            //                 StorageReadRecord::new(
            //                     StorageType::Memory,
            //                     iaddr_section_type,
            //                     iaddr,
            //                     param_index,
            //                     util::get_value_and_increase::<u64>(time_stamp)
            //                 )
            //             },
            //             1..=(1 + MAX_NUM_BYTES_FOR_LOCAL_GET) => {
            //                 StorageReadRecord::new(
            //
            //                 )
            //             }
            //     }
            // ).collect::<Vec<StorageReadRecord>>()
            // .try_into()
            // .unwrap();
    }
}