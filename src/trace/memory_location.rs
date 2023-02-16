pub enum MemoryType {

}

pub struct MemoryLocation {
    section_type: u8,
    location: u64,
}

impl MemoryLocation {
    pub fn new(section_type: u8, location: u64) -> Self {
        Self {
            section_type: section_type,
            location: location,
        }
    }
}