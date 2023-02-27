use crate::proof_context::trace::section_type::SectionType;
use crate::proof_context::trace::storage_type::StorageType;

#[derive(Debug, Clone)]
pub struct StorageReadRecord {
    storage_type: StorageType,
    section_type: SectionType,
    location: u64,
    value: u64,
    time_stamp: u64,
    // access_type: AccessType,
}

impl StorageReadRecord {
    pub fn new(storage_type: StorageType,
               section_type: SectionType,
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

    pub fn dummy(time_stamp: &u64) -> Self {
        Self::new(
            StorageType::Undefined,
            SectionType::Undefined,
            0,
            0,
            time_stamp.clone(),
        )
    }
}