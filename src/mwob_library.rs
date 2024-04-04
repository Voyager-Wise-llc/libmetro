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
pub struct MetrowerksFileObject {
    moddate: DateTime<Local>,
    file_name: String,
    full_path: String,
    obj: MetrowerksObject,
}

impl MetrowerksFileObject {
    pub fn new(file_name: &str, full_path: &str, mwob: MetrowerksObject) -> MetrowerksFileObject {
        MetrowerksFileObject {
            moddate: Local::now(),
            file_name: file_name.to_owned(),
            full_path: full_path.to_owned(),
            obj: mwob,
        }
    }

    pub fn object(&self) -> &MetrowerksObject {
        &self.obj
    }

    pub fn filename(&self) -> &str {
        self.file_name.as_str()
    }

    pub fn set_filename(&mut self, new_filename: &str) {
        self.file_name = new_filename.to_owned();
    }

    pub fn fullpath(&self) -> &str {
        self.full_path.as_str()
    }

    pub fn set_fullpath(&mut self, new_full_path: &str) {
        self.full_path = new_full_path.to_owned();
    }

    pub fn moddate(&self) -> DateTime<Local> {
        self.moddate
    }

    pub fn set_moddate(&mut self, new_moddate: &DateTime<Local>) {
        self.moddate = new_moddate.clone();
    }
}

#[derive(Debug, Clone)]
pub struct MetroWerksLibrary {
    proc: LibraryProcessor,
    flags: LibraryFlags,
    files: Vec<MetrowerksFileObject>,
}

impl Deref for MetroWerksLibrary {
    type Target = Vec<MetrowerksFileObject>;

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
        match self.proc {
            LibraryProcessor::Unknown => 0,
            LibraryProcessor::PowerPC => 1,
            LibraryProcessor::M68k => 2,
        }
    }

    pub fn new(proc: LibraryProcessor, files: &[MetrowerksFileObject]) -> MetroWerksLibrary {
        MetroWerksLibrary {
            proc: proc,
            flags: LibraryFlags::None,
            files: files.to_vec(),
        }
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
        if !match version {
            1 => proc == LibraryProcessor::PowerPC,
            2 => proc == LibraryProcessor::M68k,
            _ => false,
        } {
            return Err(format!(
                "Bad version for processor, expected {}, got {}",
                match proc {
                    LibraryProcessor::M68k => 2,
                    LibraryProcessor::PowerPC => 1,
                    LibraryProcessor::Unknown => 0,
                },
                version
            ));
        }

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

                files.push(MetrowerksFileObject {
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
            files: files,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::code_m68k::*;
    use crate::objects_m68k::BaseRegister;
    use crate::objects_m68k::NameEntry;
    use crate::objects_m68k::ObjectFlags;
    use crate::symtable_m68k::*;
    use crate::types_m68k::*;

    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::rc::Rc;

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
                ob.symbols().routines().len(),
                "Wrong number of routines, expected: {}, got: {}",
                1,
                ob.symbols().routines().len()
            );

            assert_eq!(
                3,
                ob.hunks().len(),
                "Wrong number of hunks, expected: {}, got: {}",
                3,
                ob.hunks().len()
            );

            match ob.hunks().get(1) {
                Some(hunk) => match hunk.as_ref() {
                    HunkType::GlobalCode(obj) => match obj.routine() {
                        Some(x) => {
                            let rout = x.upgrade().unwrap();
                            assert!(rout.is_function());
                            println!("{:#?}", rout);
                        }
                        None => {
                            assert!(false, "No routine attached to ObjCodeHunk");
                        }
                    },
                    _ => {
                        assert!(false, "No code hunk");
                    }
                },
                None => {
                    assert!(false, "No code hunk");
                }
            }
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
                ob.symbols().routines().len(),
                "Wrong number of routines, expected: {}, got: {}",
                2,
                ob.symbols().routines().len()
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
                ob.symbols().routines().len(),
                "Wrong number of routines, expected: {}, got: {}",
                1,
                ob.symbols().routines().len()
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

    #[test]
    fn rebuild_simple_add_and_compare() {
        // Symbol Table
        let symtab = {
            let mut symtab = SymbolTable::new();

            let add_routine = {
                let mut add_routine = Routine::new_func();

                let lvars: &mut Vec<LocalVar> = add_routine.as_mut();
                lvars.push(LocalVar::new(
                    2,
                    DataType::BasicDataType(BasicDataType::BasicTypeLong),
                    StorageKind::Value,
                    StorageClass::A7,
                    4,
                ));

                lvars.push(LocalVar::new(
                    3,
                    DataType::BasicDataType(BasicDataType::BasicTypeLong),
                    StorageKind::Value,
                    StorageClass::A7,
                    8,
                ));

                let sloc: &mut Vec<StatementLocation> = add_routine.as_mut();
                sloc.push(StatementLocation::new(0, 198));
                sloc.push(StatementLocation::new(8, 211));
                sloc.push(StatementLocation::new(-1, 211));

                add_routine
            };

            // CVW: This is kludgy
            symtab.borrow_routines_mut().push(Rc::new(add_routine));

            symtab
        };

        let hunks: CodeHunks = {
            let mut code = CodeHunks::new();

            // this already is populated with a start and end hunk
            let add_code = Hunk::new(HunkType::GlobalCode(ObjCodeHunk::new(
                1,
                173,
                ObjCodeFlag::None,
                &[32, 47, 0, 4, 208, 175, 0, 8, 78, 117],
            )));
            code.insert(1, add_code);

            code
        };

        let mwob = {
            let mut mwob = MetrowerksObject::new(&hunks, &symtab);

            // Add names
            {
                let names: &mut Vec<NameEntry> = mwob.as_mut();
                names.push(NameEntry::new(1, "add"));
                names.push(NameEntry::new(2, "a"));
                names.push(NameEntry::new(3, "b"));
            }

            // Set feature flags
            mwob.set_eightdouble(true);
            mwob.set_fourbyteint(true);
            mwob.set_basereg(BaseRegister::BaseRegA5);
            mwob.set_mc68881(false);
            mwob.set_current_version(0);
            mwob.set_old_def_version(0);
            mwob.set_old_imp_version(0);
            mwob.set_has_flags(true);
            mwob.set_object_flags(ObjectFlags::empty());

            mwob
        };

        let mfo = {
            let mfo =
                MetrowerksFileObject::new("CW11:Desktop Folder:Test:test:HelloWorld.c", "", mwob);

            mfo
        };

        let ml = MetroWerksLibrary::new(LibraryProcessor::M68k, &[mfo]);

        println!("{:#?}", ml);
    }
}
