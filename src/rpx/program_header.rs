use nom::number::complete::*;

pub struct ProgramHeader {
    pub ph_type: u32,
    pub ph_offset: u32,
    pub ph_virtual_address: u32,
    pub ph_physical_addr: u32,
    pub ph_file_size: u32,
    pub ph_mem_size: u32,
    pub ph_flags: u32,
    pub ph_align: u32,
}

impl ProgramHeader {
    pub fn new() -> ProgramHeader {
        ProgramHeader {
            ph_type: 0,
            ph_offset: 0,
            ph_virtual_address: 0,
            ph_physical_addr: 0,
            ph_file_size: 0,
            ph_mem_size: 0,
            ph_flags: 0,
            ph_align: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
