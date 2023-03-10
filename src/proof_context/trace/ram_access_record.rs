use crate::proof_context::trace::proof_type::proof_access_type::ProofAccessType;
use crate::proof_context::trace::proof_type::proof_section_type::ProofSectionType;
use crate::proof_context::trace::proof_type::proof_storage_type::ProofStorageType;

#[derive(Debug, Clone)]
pub struct RamAccessRecord {
    storage_type: ProofStorageType,
    section_type: ProofSectionType,
    location: u64,
    value: u64,
    time_stamp: u64,
    access_type: ProofAccessType,
}

impl RamAccessRecord {
    pub fn new(
        storage_type: ProofStorageType,
        section_type: ProofSectionType,
        location: u64,
        value: u64,
        time_stamp: u64,
        access_type: ProofAccessType,
    ) -> Self {
        Self {
            storage_type,
            section_type,
            location,
            value,
            time_stamp,
            access_type,
        }
    }

    pub fn dummy(time_stamp: u64) -> Self {
        Self::new(
            ProofStorageType::Undefined,
            ProofSectionType::Undefined,
            0,
            0,
            time_stamp,
            ProofAccessType::Read,
        )
    }

    pub fn get_time_stamp(&self) -> u64 {
        self.time_stamp
    }

    pub fn get_access_type(&self) -> ProofAccessType {
        self.access_type.clone()
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }
}