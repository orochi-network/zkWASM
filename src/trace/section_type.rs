use crate::memory::memory::MemorySection;

#[derive(Debug, Clone)]
pub enum SectionType {
    CodeImage = 0,
    InitialMemory = 1,
    ExecutableImage = 2,
    ProgramMemory = 3,
    Stack = 4,
    Undefined = 5,
}

impl SectionType {
    pub fn from_memory_section(memory_section: &MemorySection) -> Self {
        match memory_section {
            MemorySection::Undefined => Self::Undefined,
            MemorySection::CodeImage(_) => Self::CodeImage,
            MemorySection::InitialMemory(_) => Self::InitialMemory,
            MemorySection::ProgramMemory(_) => Self::ProgramMemory,
            MemorySection::ExecutableImage(_, _) => panic!("Cannot map executable image to SectionType")
        }
    }
}