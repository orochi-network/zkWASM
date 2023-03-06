use crate::proof_context::trace::proof_type::proof_section_type::ProofSectionType;
use crate::proof_context::trace::proof_type::proof_storage_type::ProofStorageType;

#[derive(Debug, Clone)]
pub struct StorageReadRecord {
    storage_type: ProofStorageType,
    section_type: ProofSectionType,
    location: u64,
    value: u64,
    time_stamp: u64,
    // access_type: AccessType,
}

impl StorageReadRecord {
    pub fn new(storage_type: ProofStorageType,
               section_type: ProofSectionType,
               location: u64,
               value: u64,
               time_stamp: u64,
               // access_type: AccessType
    ) -> Self {
        Self {
            storage_type,
            section_type,
            location,
            value,
            time_stamp,
            // access_type: access_type,
        }
    }

    pub fn dummy(time_stamp: u64) -> Self {
        Self::new(
            ProofStorageType::Undefined,
            ProofSectionType::Undefined,
            0,
            0,
            time_stamp,
        )
    }
}