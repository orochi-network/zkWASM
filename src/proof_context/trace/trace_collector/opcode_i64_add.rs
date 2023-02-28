use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::section_type::SectionType;
use crate::proof_context::trace::state_trace_manager::StateTraceManager;
use crate::proof_context::trace::state_trace_tuple::MAX_NUM_READ_LOCATIONS;
use crate::proof_context::trace::storage_read_record::StorageReadRecord;
use crate::proof_context::trace::storage_type::StorageType;
use crate::proof_context::trace::storage_write_record::StorageWriteRecord;

impl ProofContext {
    pub fn collect_trace_opcode_i64_add(
        &mut self,
        pc: u64,
        iaddr: u64,
        b: u64, b_location: u64,
        a: u64, a_location: u64,
        addition_result: u64, result_location: u64,
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

            res.try_into().unwrap()
        };
        todo!()
        // let write_locations = {
        //     let mut res = Vec::<StorageWriteRecord>::new();
        //     res.push(
        //         StorageWriteRecord::new(
        //
        //         )
        //     );
        //
        //     res.try_into().unwrap()
        // };
    }
}