use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::proof_type::proof_access_type::ProofAccessType;
use crate::proof_context::trace::proof_type::proof_opcode::ProofOpcode;
use crate::proof_context::trace::proof_type::proof_section_type::ProofSectionType;
use crate::proof_context::trace::proof_type::proof_storage_type::ProofStorageType;
use crate::proof_context::trace::state_trace_tuple::{MAX_NUM_RAM_ACCESS_LOCATIONS, StateTraceTuple};
use crate::proof_context::trace::ram_access_record::RamAccessRecord;
use crate::util::constant_setting::NUM_BYTES_FOR_LOCAL_GET;

impl ProofContext {
    pub fn collect_trace_opcode_local_get(
        &mut self,
        pc_before_executing: u64,
        iaddr_before_executing: u64,
        stack_depth_before_executing: usize,
        section_type_of_param_index: ProofSectionType,
        param_index: u64,
        section_types_of_read_locations: &[ProofSectionType; NUM_BYTES_FOR_LOCAL_GET],
        first_index_read: u64,
        read_bytes: &[u8; NUM_BYTES_FOR_LOCAL_GET],
        pushed_stack_value: u64,
    ) -> ProofOpcode {
        let ram_access_locations = {
            let mut res = Vec::<RamAccessRecord>::new();
            res.push(
                RamAccessRecord::new(
                    ProofStorageType::Memory,
                    section_type_of_param_index,
                    iaddr_before_executing as u64,
                    param_index,
                    self.get_time_stamp_then_increase(),
                    ProofAccessType::Read,
                )
            );

            for i in 0..NUM_BYTES_FOR_LOCAL_GET {
                res.push(
                    RamAccessRecord::new(
                        ProofStorageType::Memory,
                        section_types_of_read_locations[i].clone(),
                        first_index_read + i as u64,
                        read_bytes[i] as u64,
                        self.get_time_stamp_then_increase(),
                        ProofAccessType::Read,
                    )
                );
            }

            res.push(
                RamAccessRecord::new(
                    ProofStorageType::Stack,
                    ProofSectionType::Stack,
                    stack_depth_before_executing as u64,
                    pushed_stack_value,
                    self.get_time_stamp_then_increase(),
                    ProofAccessType::Write,
                )
            );

            let _ = (res.len()..MAX_NUM_RAM_ACCESS_LOCATIONS).into_iter().map(|_| {
                res.push(RamAccessRecord::dummy(self.get_time_stamp_then_increase()));
            });
            res.try_into().unwrap()
        };

        let proof_opcode = ProofOpcode::LocalGet;

        self.add_state_trace_tuple(
            &StateTraceTuple::new(
                pc_before_executing,
                iaddr_before_executing,
                stack_depth_before_executing,
                proof_opcode.clone(),
                ram_access_locations,
            )
        );

        proof_opcode
    }
}