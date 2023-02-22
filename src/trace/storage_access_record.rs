pub enum StorageType {
    Memory = 0,
    Stack = 1,
    Undefined = 2,
}

pub enum SectionType {
    CodeImage = 0,
    InitialMemory = 1,
    ExecutableImage = 2,
    ProgramMemory = 3,
    Stack = 4,
    Undefined = 5,
}

// pub enum AccessType {
//     Read = 0,
//     Write = 1,
// }

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
}

pub struct StorageWriteRecord {
    storage_type: StorageType,
    section_type: SectionType,
    location: u64,
    value: u64,
    time_stamp: u64,
    // access_type: AccessType,
}

impl StorageWriteRecord {
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
}