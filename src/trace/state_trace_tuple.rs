use crate::trace::storage_access_record::StorageAccessRecord;

pub const MAX_NUM_READ_LOCATIONS: usize = 3;
pub const MAX_NUM_WRITE_LOCATIONS: usize = 3;

pub struct StateTraceTuple {
    pc: u64,
    iaddr: u64,
    read_locations: [StorageAccessRecord; MAX_NUM_READ_LOCATIONS],
    write_locations: [StorageAccessRecord; MAX_NUM_WRITE_LOCATIONS],
}

impl StateTraceTuple {
    pub fn new(
        pc: u64,
        iaddr: u64,
        read_locations: [StorageAccessRecord; MAX_NUM_READ_LOCATIONS],
        write_locations: [StorageAccessRecord; MAX_NUM_WRITE_LOCATIONS],
    ) -> Self {
        Self {
            pc: pc,
            iaddr: iaddr,
            read_locations: read_locations,
            write_locations: write_locations,
        }
    }
}