use std::fs::read;
use crate::proof_context::proof_context::ProofContext;
use crate::proof_context::trace::proof_type::proof_access_type::ProofAccessType;
use crate::proof_context::trace::state_trace_tuple::StateTraceTuple;
use crate::util::constant_setting::NUM_BYTES_FOR_LOCAL_GET;
use crate::proof_context::trace::plain_trace_verification::check_constants;
use crate::proof_context::trace::ram_access_record::RamAccessRecord;

const CONCATENATING_FACTOR: u64 = 256;

impl ProofContext {
    pub fn plainly_check_opcode_local_get(
        &self,
        current_state_trace_tuple: &StateTraceTuple,
        next_state_trace_tuple: &StateTraceTuple,
    ) {
        // check iaddr and pc
        Self::check_iaddr_and_pc_consistent(current_state_trace_tuple, next_state_trace_tuple);

        let mut current_check_index: usize = 0;

        let ram_access_records = current_state_trace_tuple.get_ram_access_records();

        // first element must be reading the index
        Self::check_access_and_range_for_read_param_index(&ram_access_records[current_check_index]);
        let read_param_index = ram_access_records[current_check_index].get_value();
        current_check_index += 1;

        // next elements must be reading the bytes
        let byte_array: [u8; NUM_BYTES_FOR_LOCAL_GET] = (0..NUM_BYTES_FOR_LOCAL_GET).into_iter().map(|i| {
            Self::check_access_and_range_for_read_byte(&ram_access_records[current_check_index + i]);
            ram_access_records[current_check_index + i].get_value() as u8
        }).collect::<Vec<u8>>().try_into().unwrap();
        let first_byte_location = ram_access_records[current_check_index].get_location();
        current_check_index += NUM_BYTES_FOR_LOCAL_GET;

        // last element must be writing and value is 64 bit
        Self::check_access_and_range_for_written_value(&ram_access_records[current_check_index]);
        let written_value = ram_access_records[current_check_index].get_value();
        current_check_index += NUM_BYTES_FOR_LOCAL_GET;

        // location of first read_byte must be equal index_param
        assert_eq!(self.get_param_start() + read_param_index * 8, first_byte_location);

        // check the computation from byte to the written value is consistent
        let mut concatenated_value = 0;
        for value in byte_array {
            concatenated_value = concatenated_value * CONCATENATING_FACTOR + value as u64;
        }
        assert_eq!(written_value, concatenated_value);
    }

    fn check_access_and_range_for_read_param_index(index_record: &RamAccessRecord) {
        assert_eq!(index_record.get_access_type(), ProofAccessType::Read);
        Self::verify_in_range(
            0, check_constants::MAX_VALUE_64_BIT,
            index_record.get_value()
        );
    }

    fn check_access_and_range_for_read_byte(byte_record: &RamAccessRecord) {
        assert_eq!(
            byte_record.get_access_type(),
            ProofAccessType::Read
        );
        Self::verify_in_range(
            0, check_constants::MAX_VALUE_8_BIT,
            byte_record.get_value()
        );
    }

    fn check_access_and_range_for_written_value(written_value_record: &RamAccessRecord) {
        assert_eq!(
            written_value_record.get_access_type(),
            ProofAccessType::Write
        );
        Self::verify_in_range(
            0, check_constants::MAX_VALUE_64_BIT,
            written_value_record.get_value()
        );
    }

    fn check_iaddr_and_pc_consistent(
        current_state_trace_tuple: &StateTraceTuple,
        next_state_trace_tuple: &StateTraceTuple,
    ) {
        assert_eq!(
            current_state_trace_tuple.get_iaddr_before_executing() + 2, // since we need to keep param_index
            next_state_trace_tuple.get_iaddr_before_executing()
        );
        assert_eq!(
            current_state_trace_tuple.get_pc_before_executing() + 1,
            next_state_trace_tuple.get_pc_before_executing()
        );
    }
}