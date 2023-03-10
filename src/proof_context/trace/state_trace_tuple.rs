use crate::proof_context::trace::proof_type::proof_opcode::ProofOpcode;
use crate::proof_context::trace::ram_access_record::RamAccessRecord;

pub const MAX_NUM_RAM_ACCESS_RECORDS: usize = 10;

#[derive(Clone)]
pub struct StateTraceTuple {
    pc_before_executing: u64,
    iaddr_before_executing: u64,
    stack_depth_before_executing: usize,
    proof_opcode: ProofOpcode,
    ram_access_records: [RamAccessRecord; MAX_NUM_RAM_ACCESS_RECORDS],
}

impl StateTraceTuple {
    pub fn new(
        pc_before_executing: u64,
        iaddr_before_executing: u64,
        stack_depth_before_executing: usize,
        proof_opcode: ProofOpcode,
        ram_access_records: [RamAccessRecord; MAX_NUM_RAM_ACCESS_RECORDS],
    ) -> Self {
        Self {
            pc_before_executing,
            iaddr_before_executing,
            stack_depth_before_executing,
            proof_opcode,
            ram_access_records,
        }
    }

    pub fn get_proof_opcode(&self) -> ProofOpcode {
        self.proof_opcode.clone()
    }

    pub fn get_ram_access_records(&self) -> &[RamAccessRecord; MAX_NUM_RAM_ACCESS_RECORDS] {
        &self.ram_access_records
    }

    pub fn get_pc_before_executing(&self) -> u64 {
        self.pc_before_executing
    }

    pub fn get_iaddr_before_executing(&self) -> u64 {
        self.iaddr_before_executing
    }
}