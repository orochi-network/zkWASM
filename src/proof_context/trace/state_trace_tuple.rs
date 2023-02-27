use crate::proof_context::trace::storage_read_record::StorageReadRecord;
use crate::proof_context::trace::storage_write_record::StorageWriteRecord;

pub const MAX_NUM_READ_LOCATIONS: usize = 9;
pub const MAX_NUM_WRITE_LOCATIONS: usize = 2;

#[derive(Clone)]
pub struct StateTraceTuple {
    pc_before_executing: u64,
    iaddr_before_executing: u64,
    stack_depth_before_executing: usize,
    read_locations: [StorageReadRecord; MAX_NUM_READ_LOCATIONS],
    write_locations: [StorageWriteRecord; MAX_NUM_WRITE_LOCATIONS],
}

impl StateTraceTuple {
    pub fn new(
        pc_before_executing: u64,
        iaddr_before_executing: u64,
        stack_depth_before_executing: usize,
        read_locations: [StorageReadRecord; MAX_NUM_READ_LOCATIONS],
        write_locations: [StorageWriteRecord; MAX_NUM_WRITE_LOCATIONS],
    ) -> Self {
        Self {
            pc_before_executing,
            iaddr_before_executing,
            stack_depth_before_executing,
            read_locations,
            write_locations,
        }
    }
}