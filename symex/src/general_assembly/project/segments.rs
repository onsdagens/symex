//! A loader that can load all segments from a elf file properly.

use object::{read::elf::ProgramHeader, File, Object};
#[derive(Debug)]
pub struct Segment {
    data: Vec<u8>,
    start_address: u64,
    end_address: u64,
}

#[derive(Debug)]
pub struct Segments(Vec<Segment>);

impl Segments {
    pub fn from_single_segment(data: Vec<u8>, start_addr: u64, end_addr: u64) -> Self {
        Segments(vec![Segment {
            data,
            start_address: start_addr,
            end_address: end_addr,
        }])
    }

    pub fn from_file(file: &File) -> Self {
        let elf_file = match file {
            File::Elf32(elf_file) => elf_file,
            File::Elf64(_elf_file) => todo!(),
            _ => todo!(),
        };

        let mut ret = vec![];
        for segment in elf_file.raw_segments() {
            let segment_type = segment.p_type.get(file.endianness());
            //println!("flags: {:?}", segment.p_flags(file.endianness()));
            //println!("vaddr: {:?}", segment.p_vaddr(file.endianness()));
            //println!("type: {:?}", segment_type);
            // here, we check if segment is executable.
            // otherwise, we will init static variables, which gives constant values to static
            // muts, obviously that's no good.
            if segment_type == 1 && ((segment.p_flags(file.endianness()) & 0b1) == 1) {
                //println!("loading");
                // if it is a LOAD segment
                let addr_start = segment.p_vaddr.get(file.endianness()) as u64;
                //let size = segment.p_memsz.get(file.endianness());
                let data = segment.data(file.endianness(), elf_file.data()).unwrap();

                ret.push(Segment {
                    data: data.to_owned(),
                    start_address: addr_start,
                    end_address: addr_start + data.len() as u64,
                })
            }
        }
        Segments(ret)
    }

    pub fn read_raw_bytes(&self, address: u64, bytes: usize) -> Option<&[u8]> {
        for segment in &self.0 {
            if address >= segment.start_address && address < segment.end_address {
                let offset = (address - segment.start_address) as usize;
                let data_slice = &segment.data[offset..(offset + bytes)];
                return Some(data_slice);
            }
        }

        None
    }
}
