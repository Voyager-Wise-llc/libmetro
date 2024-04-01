use crate::objects_m68k::MetrowerksObject;

use super::util;
use std::ffi::CStr;
use std::slice::Iter;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LibraryMagicWord {
    LibraryMagicWord = 0x4d574f42,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LibraryProcessor {
    Unknown = 0,
    PowerPC = 0x50504320,
    M68k = 0x4d36384b,
}

impl From<u32> for LibraryProcessor {
    fn from(value: u32) -> Self {
        match value {
            x if x == LibraryProcessor::M68k as u32 => LibraryProcessor::M68k,
            x if x == LibraryProcessor::PowerPC as u32 => LibraryProcessor::PowerPC,
            _ => LibraryProcessor::Unknown,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LibraryFlags {
    None = 0,
}

impl LibraryFlags {
    fn default() -> LibraryFlags {
        LibraryFlags::None
    }
}

#[derive(Debug, Clone)]
pub struct FileObject {
    moddate: u32,
    file_name: String,
    full_path: String,
    obj: MetrowerksObject,
}

impl FileObject {
    pub fn object(&self) -> &MetrowerksObject {
        &self.obj
    }

    pub fn filename(&self) -> &str {
        self.file_name.as_str()
    }

    pub fn fullpath(&self) -> &str {
        self.full_path.as_str()
    }

    pub fn moddate(&self) -> u32 {
        self.moddate
    }
}

#[derive(Debug, Clone)]
pub struct MetroWerksLibrary {
    proc: LibraryProcessor,
    flags: LibraryFlags,
    version: u32,
    files: Vec<FileObject>,
}

impl MetroWerksLibrary {
    #[inline(always)]
    pub fn proc(&self) -> LibraryProcessor {
        self.proc
    }

    #[inline(always)]
    pub fn flags(&self) -> LibraryFlags {
        self.flags
    }

    #[inline(always)]
    pub fn version(&self) -> u32 {
        self.version
    }

    #[inline(always)]
    pub fn file_iter(&self) -> Iter<FileObject> {
        self.files.iter()
    }

    #[inline(always)]
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

#[derive(PartialEq)]
enum LibraryParseState {
    Start,
    ParseLibraryHeader,

    ParseFile,

    End,
}

impl Default for LibraryParseState {
    fn default() -> Self {
        LibraryParseState::Start
    }
}

pub fn parse_library(value: &[u8]) -> Result<MetroWerksLibrary, String> {
    let mut data: &[u8] = value;
    let mut files: Vec<FileObject> = Vec::new();
    let mut remaining_files = 0;

    let mut proc = LibraryProcessor::Unknown;
    let mut flags = LibraryFlags::default();
    let mut version: u32 = 0;

    let mut state: LibraryParseState = LibraryParseState::default();
    while state != LibraryParseState::End {
        state = match state {
            LibraryParseState::Start => LibraryParseState::ParseLibraryHeader,
            LibraryParseState::ParseLibraryHeader => {
                let magic = util::convert_be_u32(&data[0..4].try_into().unwrap());

                if magic != LibraryMagicWord::LibraryMagicWord as u32 {
                    return Err(format!(
                        "Bad Magic Word: Expected: {}, got: {}",
                        LibraryMagicWord::LibraryMagicWord as u32,
                        magic
                    ));
                }

                let proc_u32 = util::convert_be_u32(&data[4..8].try_into().unwrap());
                proc = LibraryProcessor::from(proc_u32);

                let flags_u32 = util::convert_be_u32(&data[8..12].try_into().unwrap());
                if flags_u32 != 0 {
                    return Err(format!("Bad flags for header, got: {}", flags_u32));
                }
                flags = LibraryFlags::None;

                version = util::convert_be_u32(&data[12..16].try_into().unwrap());

                let num_objects = util::convert_be_u32(&data[24..28].try_into().unwrap());

                if num_objects != 0 {
                    data = &data[28..];
                    remaining_files = num_objects;
                    LibraryParseState::ParseFile
                } else {
                    LibraryParseState::End
                }
            }

            LibraryParseState::ParseFile => {
                let file_moddate = util::convert_be_u32(&data[0..4].try_into().unwrap());
                let file_name_loc = util::convert_be_u32(&data[4..8].try_into().unwrap()) as usize;
                let full_path_loc = util::convert_be_u32(&data[8..12].try_into().unwrap()) as usize;
                let data_start: usize =
                    util::convert_be_u32(&data[12..16].try_into().unwrap()) as usize;
                let data_size: usize =
                    util::convert_be_u32(&data[16..20].try_into().unwrap()) as usize;

                // The file_name, full_path, and bytes are relative to the LIBRARY Header not the FILE Header
                let file_name = CStr::from_bytes_until_nul(&value[file_name_loc..])
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned();

                let full_path: String = if full_path_loc == 0 {
                    String::new()
                } else {
                    CStr::from_bytes_until_nul(&value[full_path_loc as usize..])
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned()
                };

                // The bytes are relative to the LIBRARY Header not the FILE Header
                let bytes = &value[data_start..(data_start + data_size)];
                data = &data[20..];

                files.push(FileObject {
                    moddate: file_moddate,
                    file_name: file_name,
                    full_path: full_path,
                    obj: MetrowerksObject::try_from(bytes)?,
                });

                remaining_files -= 1;

                if remaining_files == 0 {
                    LibraryParseState::End
                } else {
                    LibraryParseState::ParseFile
                }
            }

            _ => todo!(),
        }
    }

    Ok(MetroWerksLibrary {
        proc: proc,
        flags: flags,
        version: version,
        files: files,
    })
}

impl TryFrom<&[u8]> for MetroWerksLibrary {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        parse_library(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_simple_library() {
        let mut lib = File::open("test/data/add.lib.metro").unwrap();
        let mut ve: Vec<u8> = vec![];
        lib.read_to_end(&mut ve).unwrap();

        let l = MetroWerksLibrary::try_from(ve.as_ref()).unwrap();

        println!("{:#?}", l);

        for f in l.file_iter() {
            let ob = f.object();

            assert_eq!(
                3,
                ob.names().len(),
                "Wrong number of names, expected: {}, got: {}",
                3,
                ob.names().len()
            );

            assert_eq!(
                1,
                ob.symbols().unwrap().routines().len(),
                "Wrong number of routines, expected: {}, got: {}",
                1,
                ob.symbols().unwrap().routines().len()
            );

            assert_eq!(
                3,
                ob.hunks().len(),
                "Wrong number of hunks, expected: {}, got: {}",
                3,
                ob.hunks().len()
            );
        }
    }
}
