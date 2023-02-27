use crate::proof_context::trace::section_type::SectionType;
use crate::proof_context::trace::storage_type::StorageType;

#[derive(Debug, Clone)]
pub struct StorageWriteRecord {
    storage_type: StorageType,
    section_type: SectionType,
    location: u64,
    value: u64,
    time_stamp: u64,
}

impl StorageWriteRecord {
    pub fn new(
        storage_type: StorageType,
        section_type: SectionType,
        location: u64,
        value: u64,
        time_stamp: u64,
    ) -> Self {
        Self {
            storage_type,
            section_type,
            location,
            value,
            time_stamp,
        }
    }
}