use crate::proof_context::trace::proof_type::proof_opcode::ProofOpcode;
use crate::proof_context::trace::storage_read_record::StorageReadRecord;
use crate::proof_context::trace::storage_write_record::StorageWriteRecord;

pub const MAX_NUM_READ_LOCATIONS: usize = 9;
pub const MAX_NUM_WRITE_LOCATIONS: usize = 8;

#[derive(Clone)]
pub struct StateTraceTuple {
    pc_before_executing: u64,
    iaddr_before_executing: u64,
    stack_depth_before_executing: usize,
    proof_opcode: ProofOpcode,
    read_locations: [StorageReadRecord; MAX_NUM_READ_LOCATIONS],
    write_locations: [StorageWriteRecord; MAX_NUM_WRITE_LOCATIONS],
}

impl StateTraceTuple {
    pub fn new(
        pc_before_executing: u64,
        iaddr_before_executing: u64,
        stack_depth_before_executing: usize,
        proof_opcode: ProofOpcode,
        read_locations: [StorageReadRecord; MAX_NUM_READ_LOCATIONS],
        write_locations: [StorageWriteRecord; MAX_NUM_WRITE_LOCATIONS],
    ) -> Self {
        Self {
            pc_before_executing,
            iaddr_before_executing,
            stack_depth_before_executing,
            proof_opcode,
            read_locations,
            write_locations,
        }
    }

    pub fn get_proof_opcode(&self) -> ProofOpcode {
        self.proof_opcode.clone()
    }

    pub fn get_read_locations(&self) -> &[StorageReadRecord; MAX_NUM_READ_LOCATIONS] {
        &self.read_locations
    }

    pub fn get_write_locations(&self) -> &[StorageWriteRecord; MAX_NUM_WRITE_LOCATIONS] {
        &self.write_locations
    }
}