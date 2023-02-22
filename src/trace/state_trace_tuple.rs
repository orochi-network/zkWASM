use crate::trace::storage_access_record::{
    StorageReadRecord, StorageWriteRecord
};

pub const MAX_NUM_READ_LOCATIONS: usize = 9;
pub const MAX_NUM_WRITE_LOCATIONS: usize = 9;

pub struct StateTraceTuple {
    pc: u64,
    iaddr: u64,
    read_locations: [StorageReadRecord; MAX_NUM_READ_LOCATIONS],
    write_locations: [StorageWriteRecord; MAX_NUM_WRITE_LOCATIONS],
}

impl StateTraceTuple {
    pub fn new(
        pc: u64,
        iaddr: u64,
        read_locations: [StorageReadRecord; MAX_NUM_READ_LOCATIONS],
        write_locations: [StorageWriteRecord; MAX_NUM_WRITE_LOCATIONS],
    ) -> Self {
        Self {
            pc: pc,
            iaddr: iaddr,
            read_locations: read_locations,
            write_locations: write_locations,
        }
    }
}