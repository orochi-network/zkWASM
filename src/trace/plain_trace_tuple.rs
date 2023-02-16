use crate::trace::memory_location::MemoryLocation;

const MAX_NUM_READ_LOCATIONS: usize = 3;
const MAX_NUM_WRITE_LOCATIONS: usize = 3;

pub struct PlainTraceTuple {
    pc: u64,
    iaddr: u64,
    read_locations: [MemoryLocation; MAX_NUM_READ_LOCATIONS],
    write_locations: [MemoryLocation; MAX_NUM_WRITE_LOCATIONS],
}

impl PlainTraceTuple {
    pub fn new(
        pc: u64,
        iaddr: u64,
        read_locations: [MemoryLocation; MAX_NUM_READ_LOCATIONS],
        write_locations: [MemoryLocation; MAX_NUM_WRITE_LOCATIONS],
    ) -> Self {
        Self {
            pc: pc,
            iaddr: iaddr,
            read_locations: read_locations,
            write_locations: write_locations,
        }
    }
}