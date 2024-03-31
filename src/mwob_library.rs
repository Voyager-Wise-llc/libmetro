use super::util;
use std::ffi::CStr;
use std::slice::Iter;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum LibraryMagicWord {
    LibraryMagicWord = 0x4d574f42,
}

impl Default for LibraryMagicWord {
    fn default() -> Self {
        LibraryMagicWord::LibraryMagicWord
    }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq)]
pub enum LibraryProcessor {
    Unknown = 0,
    PowerPC = 0x50504320,
    M68k = 0x4d36384b,
}

impl Default for LibraryProcessor {
    fn default() -> Self {
        LibraryProcessor::Unknown
    }
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
#[derive(Clone, Copy, PartialEq)]
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
    data: Vec<u8>,
}

impl Default for FileObject {
    fn default() -> Self {
        Self {
            moddate: 0,
            file_name: String::new(),
            full_path: String::new(),
            data: vec![],
        }
    }
}

impl FileObject {
    fn new(moddate: u32, file_name: String, full_path: String, data: &[u8]) -> Self {
        Self {
            moddate: moddate,
            file_name: file_name,
            full_path: full_path,
            data: data.to_vec(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn length(&self) -> usize {
        self.data.len()
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

pub struct MetroWerksLibrary {
    proc: LibraryProcessor,
    flags: LibraryFlags,
    version: u32,
    files: Vec<FileObject>,
}

impl MetroWerksLibrary {
    fn new(
        proc: LibraryProcessor,
        flags: LibraryFlags,
        version: u32,
        files: Vec<FileObject>,
    ) -> Self {
        Self {
            proc: proc,
            flags: flags,
            version: version,
            files: files,
        }
    }

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

    FileStart,
    CommitFile,

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
    let mut current_file = FileObject::default();
    let mut remaining_files = 0;

    let mut proc = LibraryProcessor::default();
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
                    LibraryParseState::FileStart
                } else {
                    LibraryParseState::End
                }
            }

            LibraryParseState::FileStart => {
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

                current_file = FileObject::new(file_moddate, file_name, full_path, bytes);

                LibraryParseState::CommitFile
            }
            LibraryParseState::CommitFile => {
                files.push(current_file.clone());
                remaining_files -= 1;

                if remaining_files == 0 {
                    LibraryParseState::End
                } else {
                    LibraryParseState::FileStart
                }
            }
            _ => todo!(),
        }
    }

    Ok(MetroWerksLibrary::new(proc, flags, version, files))
}

impl TryFrom<&[u8]> for MetroWerksLibrary {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        parse_library(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::objects_m68k::MetrowerksObject;

    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_simple_library() {
        let mut lib = File::open("test/data/add.lib.metro").unwrap();
        let mut ve: Vec<u8> = vec![];
        lib.read_to_end(&mut ve).unwrap();

        let l = MetroWerksLibrary::try_from(ve.as_ref()).unwrap();

        println!("{} objects.", l.file_count());

        for raw_file in l.file_iter() {
            let ob = MetrowerksObject::try_from(raw_file.as_bytes()).unwrap();
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
        }
    }
}
