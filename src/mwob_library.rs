use chrono::{DateTime, Local};

use crate::objects_m68k::MetrowerksObject;

use super::util;
use std::ffi::CStr;
use std::ops::Deref;

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

#[derive(Debug, Clone)]
pub struct FileObject {
    moddate: DateTime<Local>,
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

    pub fn moddate(&self) -> DateTime<Local> {
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

impl Deref for MetroWerksLibrary {
    type Target = Vec<FileObject>;

    fn deref(&self) -> &Self::Target {
        &self.files
    }
}

impl MetroWerksLibrary {
    pub fn proc(&self) -> LibraryProcessor {
        self.proc
    }

    pub fn flags(&self) -> LibraryFlags {
        self.flags
    }

    pub fn version(&self) -> u32 {
        self.version
    }
}

impl TryFrom<&[u8]> for MetroWerksLibrary {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let magic = util::convert_be_u32(&value[0..4].try_into().unwrap());

        if magic != LibraryMagicWord::LibraryMagicWord as u32 {
            return Err(format!(
                "Bad Magic Word: Expected: {}, got: {}",
                LibraryMagicWord::LibraryMagicWord as u32,
                magic
            ));
        }

        let proc_u32 = util::convert_be_u32(&value[4..8].try_into().unwrap());
        let proc = LibraryProcessor::from(proc_u32);

        let flags_u32 = util::convert_be_u32(&value[8..12].try_into().unwrap());
        if flags_u32 != 0 {
            return Err(format!("Bad flags for header, got: {}", flags_u32));
        }
        let flags = LibraryFlags::None;

        let version = util::convert_be_u32(&value[12..16].try_into().unwrap());

        let num_files = util::convert_be_u32(&value[24..28].try_into().unwrap());

        let files = if num_files != 0 {
            let mut obj_bytes = &value[28..];
            let mut remaining_files = num_files;
            let mut files = vec![];

            while remaining_files > 0 {
                let file_moddate = util::convert_be_u32(&obj_bytes[0..4].try_into().unwrap());
                let file_name_loc =
                    util::convert_be_u32(&obj_bytes[4..8].try_into().unwrap()) as usize;
                let full_path_loc =
                    util::convert_be_u32(&obj_bytes[8..12].try_into().unwrap()) as usize;
                let data_start: usize =
                    util::convert_be_u32(&obj_bytes[12..16].try_into().unwrap()) as usize;
                let data_size: usize =
                    util::convert_be_u32(&obj_bytes[16..20].try_into().unwrap()) as usize;

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
                obj_bytes = &obj_bytes[20..];

                files.push(FileObject {
                    moddate: util::from_mac_datetime(file_moddate).into(),
                    file_name: file_name,
                    full_path: full_path,
                    obj: MetrowerksObject::try_from(bytes)?,
                });

                remaining_files -= 1;
            }

            files
        } else {
            vec![]
        };

        Ok(MetroWerksLibrary {
            proc: proc,
            flags: flags,
            version: version,
            files: files,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_simple_add_library() {
        let mut lib = File::open("test/data/add.lib.metro").unwrap();
        let mut ve: Vec<u8> = vec![];
        lib.read_to_end(&mut ve).unwrap();

        let lut = MetroWerksLibrary::try_from(ve.as_ref()).unwrap();

        println!("{:#?}", lut);

        for f in lut.iter() {
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

    #[test]
    fn test_simple_multi_func_library() {
        let mut lib = File::open("test/data/two_funcs.lib.metro").unwrap();
        let mut ve: Vec<u8> = vec![];
        lib.read_to_end(&mut ve).unwrap();

        let lut = MetroWerksLibrary::try_from(ve.as_ref()).unwrap();

        println!("{:#?}", lut);

        for f in lut.iter() {
            let ob = f.object();

            assert_eq!(
                4,
                ob.names().len(),
                "Wrong number of names, expected: {}, got: {}",
                4,
                ob.names().len()
            );

            assert_eq!(
                2,
                ob.symbols().unwrap().routines().len(),
                "Wrong number of routines, expected: {}, got: {}",
                2,
                ob.symbols().unwrap().routines().len()
            );

            assert_eq!(
                4,
                ob.hunks().len(),
                "Wrong number of hunks, expected: {}, got: {}",
                4,
                ob.hunks().len()
            );
        }
    }

    #[test]
    fn test_cw_set_volume_example_library() {
        let mut lib = File::open("test/data/set_volume_ex.lib.metro").unwrap();
        let mut ve: Vec<u8> = vec![];
        lib.read_to_end(&mut ve).unwrap();

        let lut = MetroWerksLibrary::try_from(ve.as_ref()).unwrap();

        println!("{:#?}", lut);

        for f in lut.iter() {
            let ob = f.object();

            assert_eq!(
                2,
                ob.names().len(),
                "Wrong number of names, expected: {}, got: {}",
                2,
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
                5,
                ob.hunks().len(),
                "Wrong number of hunks, expected: {}, got: {}",
                5,
                ob.hunks().len()
            );
        }
    }
}
