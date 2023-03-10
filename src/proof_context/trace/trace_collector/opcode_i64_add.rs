use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::proof_type::proof_access_type::ProofAccessType;
use crate::proof_context::trace::proof_type::proof_opcode::ProofOpcode;
use crate::proof_context::trace::proof_type::proof_section_type::ProofSectionType;
use crate::proof_context::trace::proof_type::proof_storage_type::ProofStorageType;
use crate::proof_context::trace::state_trace_tuple::{MAX_NUM_RAM_ACCESS_RECORDS, StateTraceTuple};
use crate::proof_context::trace::ram_access_record::RamAccessRecord;

impl ProofContext {
    pub fn collect_trace_opcode_i64_add(
        &mut self,
        pc_before_executing: u64,
        iaddr_before_executing: u64,
        stack_depth_before_executing: usize,
        b_location: u64, b: u64,
        a_location: u64, a: u64,
        addition_result_in_section_types: [ProofSectionType; 8],
        addition_result_starting_location: u64,
        addition_result_in_bytes: [u8; 8],
    ) -> ProofOpcode {
        let ram_access_records: [RamAccessRecord; MAX_NUM_RAM_ACCESS_RECORDS] = {
            let mut res = Vec::<RamAccessRecord>::new();
            res.push(
                RamAccessRecord::new(
                    ProofStorageType::Stack,
                    ProofSectionType::Stack,
                    b_location,
                    b,
                    self.get_time_stamp_then_increase(),
                    ProofAccessType::Read,
                )
            );

            res.push(
                RamAccessRecord::new(
                    ProofStorageType::Stack,
                    ProofSectionType::Stack,
                    a_location,
                    a,
                    self.get_time_stamp_then_increase(),
                    ProofAccessType::Read,
                )
            );

            for i in 0..8 {
                res.push(
                    RamAccessRecord::new(
                        ProofStorageType::Memory,
                        addition_result_in_section_types[i].clone(),
                        addition_result_starting_location + i as u64,
                        addition_result_in_bytes[i] as u64,
                        self.get_time_stamp_then_increase(),
                        ProofAccessType::Write,
                    )
                );
            }

            res.try_into().unwrap()
        };

        let proof_opcode = ProofOpcode::I64Add;

        self.add_state_trace_tuple(
            &StateTraceTuple::new(
                pc_before_executing,
                iaddr_before_executing,
                stack_depth_before_executing,
                proof_opcode.clone(),
                ram_access_records,
            )
        );

        proof_opcode
    }
}