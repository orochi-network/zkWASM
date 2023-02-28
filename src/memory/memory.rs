pub type SectionSize = (u64, u64);

#[derive(Clone, PartialEq)]
pub enum MemorySection {
    Undefined,
    // Read only memory that contain Wasm opcode
    CodeImage(SectionSize),
    // Read only memory, initial memory that contain initial state
    InitialMemory(SectionSize),
    // Read only memory that contain CodeImage and Initial Memory
    ExecutableImage(SectionSize, SectionSize),
    // Writable memory of program
    ProgramMemory(SectionSize),
}

const NUM_SECTIONS: usize = 3;

pub trait Memory<'a> {
    fn init(&mut self, data: &[u8], offset: u64, length: u64) -> u64;
    fn write(&mut self, data: &[u8], offset: u64, length: u64) -> u64;
    fn read(&'a self, offset: u64, length: u64) -> &'a [u8];
    fn get_section_from_offset(&self, offset: u64) -> MemorySection;
    fn size(&self) -> u64;
}

pub struct WasmMemory {
    memory: Vec<u8>,
    sections: Vec<MemorySection>,
}

impl WasmMemory {
    pub fn new(size: u64) -> Self {
        let mut memory = Vec::new();
        let mut sections = Vec::new();
        memory.resize(size as usize, 0);
        sections.resize(NUM_SECTIONS, MemorySection::Undefined);
        WasmMemory { memory, sections }
    }

    pub fn add_section(&mut self, section: MemorySection) {
        match section {
            MemorySection::CodeImage(s) => self.sections[0] = MemorySection::CodeImage(s),
            MemorySection::InitialMemory(s) => self.sections[1] = MemorySection::InitialMemory(s),
            MemorySection::ProgramMemory(s) => self.sections[2] = MemorySection::ProgramMemory(s),
            MemorySection::ExecutableImage(s, v) => {
                self.sections[0] = MemorySection::CodeImage(s);
                self.sections[1] = MemorySection::InitialMemory(v);
            }
            _ => panic!("Undefined memory section"),
        }
    }

    pub fn get_section_size_from_section_index(&self, index: usize) -> SectionSize {
        self.get_section_size_from_section(&self.sections[index])
    }

    pub fn get_section_size_from_section(&self, section: &MemorySection) -> SectionSize {
        match *section {
            MemorySection::CodeImage(s) => s,
            MemorySection::InitialMemory(s) => s,
            MemorySection::ProgramMemory(s) => s,
            _ => panic!("Undefined memory section"),
        }
    }
}

impl<'a> Memory<'a> for WasmMemory {
    // @TODO we should lock memory after init
    fn init(&mut self, data: &[u8], offset: u64, length: u64) -> u64 {
        for i in 0..length {
            self.memory[(i + offset) as usize] = data[i as usize];
        }
        offset
    }

    fn write(&mut self, data: &[u8], offset: u64, length: u64) -> u64 {
        match self.sections[2] {
            MemorySection::ProgramMemory(s) => {
                let (start, end) = s;
                if offset < start || (offset + length) > end {
                    panic!("Write to the wrong memory section")
                }
            }
            _ => panic!("Invalid memory section"),
        }
        for i in 0..length {
            self.memory[(i + offset) as usize] = data[i as usize];
        }
        offset
    }

    fn read(&'a self, offset: u64, length: u64) -> &'a [u8] {
        &self.memory[offset as usize..(offset + length) as usize]
    }

    fn get_section_from_offset(&self, offset: u64) -> MemorySection {
        let mut res = MemorySection::Undefined;
        let mut largest_reached_offset: u64 = 0;
        for section in &self.sections {
            if *section != MemorySection::Undefined {
                if res == MemorySection::Undefined {
                    res = section.clone();
                } else {
                    let (section_start_offset, _) = self.get_section_size_from_section(&section);
                    if section_start_offset <= offset && largest_reached_offset < section_start_offset {
                        largest_reached_offset = section_start_offset;
                        res = section.clone();
                    }
                }
            }
        }

        res
    }

    fn size(&self) -> u64 {
        self.memory.len() as u64
    }
}
