use super::util;
use std::ffi::CStr;
use std::ffi::CString;
use std::slice::Iter;

#[derive(PartialEq)]
pub enum LibraryMagicWord {
    LibraryMagicWord = 0x4d574f42,
}

impl Default for LibraryMagicWord {
    fn default() -> Self {
        LibraryMagicWord::LibraryMagicWord
    }
}

#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub enum LibraryHeaderFlags {
    None = 0,
}

pub struct LibraryHeader {
    proc: LibraryProcessor,
    flags: LibraryHeaderFlags,
    version: u32,
    code_size: u32,
    data_size: u32,
    num_object_files: u32,
}

impl Default for LibraryHeader {
    fn default() -> Self {
        Self {
            proc: LibraryProcessor::Unknown,
            flags: LibraryHeaderFlags::None,
            version: 0,
            code_size: 0,
            data_size: 0,
            num_object_files: 0,
        }
    }
}

impl LibraryHeader {
    pub fn proc(self, proc: LibraryProcessor) -> Self {
        Self {
            proc: proc,
            flags: self.flags,
            version: self.version,
            code_size: self.code_size,
            data_size: self.data_size,
            num_object_files: self.num_object_files,
        }
    }

    pub fn flags(self, flags: LibraryHeaderFlags) -> Self {
        Self {
            proc: self.proc,
            flags: flags,
            version: self.version,
            code_size: self.code_size,
            data_size: self.data_size,
            num_object_files: self.num_object_files,
        }
    }

    pub fn version(self, version: u32) -> Self {
        Self {
            proc: self.proc,
            flags: self.flags,
            version: version,
            code_size: self.code_size,
            data_size: self.data_size,
            num_object_files: self.num_object_files,
        }
    }

    pub fn code_size(self, code_size: u32) -> Self {
        Self {
            proc: self.proc,
            flags: self.flags,
            version: self.version,
            code_size: code_size,
            data_size: self.data_size,
            num_object_files: self.num_object_files,
        }
    }

    pub fn data_size(self, data_size: u32) -> Self {
        Self {
            proc: self.proc,
            flags: self.flags,
            version: self.version,
            code_size: self.code_size,
            data_size: data_size,
            num_object_files: self.num_object_files,
        }
    }

    pub fn num_object_files(self, num_object_files: u32) -> Self {
        Self {
            proc: self.proc,
            flags: self.flags,
            version: self.version,
            code_size: self.code_size,
            data_size: self.data_size,
            num_object_files: num_object_files,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileHeader {
    moddate: u32,
    filename: CString,
    fullpathname: CString,
    object_start: u32,
    object_size: u32,
}

impl FileHeader {
    pub fn new() -> Self {
        Self {
            moddate: 0,
            filename: CString::new("").unwrap(),
            fullpathname: CString::new("").unwrap(),
            object_start: 0,
            object_size: 0,
        }
    }
    pub fn mod_date(self, moddate: u32) -> Self {
        Self {
            moddate: moddate,
            filename: self.filename,
            fullpathname: self.fullpathname,
            object_start: self.object_start,
            object_size: self.object_size,
        }
    }

    pub fn filename(self, filename: CString) -> Self {
        Self {
            moddate: self.moddate,
            filename: filename.clone(),
            fullpathname: self.fullpathname,
            object_start: self.object_start,
            object_size: self.object_size,
        }
    }

    pub fn fullpathname(self, fullpathname: CString) -> Self {
        Self {
            moddate: self.moddate,
            filename: self.filename,
            fullpathname: fullpathname.clone(),
            object_start: self.object_start,
            object_size: self.object_size,
        }
    }

    pub fn object_start(self, object_start: u32) -> Self {
        Self {
            moddate: self.moddate,
            filename: self.filename,
            fullpathname: self.fullpathname,
            object_start: object_start,
            object_size: self.object_size,
        }
    }

    pub fn object_size(self, object_size: u32) -> Self {
        Self {
            moddate: self.moddate,
            filename: self.filename,
            fullpathname: self.fullpathname,
            object_start: self.object_start,
            object_size: object_size,
        }
    }

    pub fn start(&self) -> usize {
        self.object_start as usize
    }

    pub fn end(&self) -> usize {
        (self.object_start + self.object_size) as usize
    }

    pub fn length(&self) -> usize {
        self.object_size as usize
    }
}

#[derive(Debug, Clone)]
pub struct FileObject {
    header: FileHeader,
    data: Vec<u8>,
}

impl FileObject {
    pub fn new(header: FileHeader, data: &[u8]) -> Self {
        Self {
            header: header,
            data: data.to_vec(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn filename(&self) -> &CStr {
        self.header.filename.as_c_str()
    }
}

pub struct MetroWerksLibrary {
    header: LibraryHeader,
    files: Vec<FileObject>,
}

impl MetroWerksLibrary {
    pub fn new(header: LibraryHeader, files: Vec<FileObject>) -> Self {
        Self {
            header: header,
            files: files,
        }
    }

    #[inline(always)]
    pub fn file_iter(&self) -> Iter<FileObject> {
        self.files.iter()
    }

    #[inline(always)]
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    pub fn header(&self) -> &LibraryHeader {
        &self.header
    }
}

#[derive(PartialEq)]
enum LibraryParseState {
    LibraryHeaderStart,
    LibraryHeaderMagicWord,
    LibraryHeaderProc,
    LibraryHeaderFlags,
    LibraryHeaderVersion,
    LibraryHeaderCodeSize,
    LibraryHeaderDataSize,
    LibraryHeaderNumObjFiles,

    FileStart,
    FileHeaderModDate,
    FileHeaderFileName,
    FileHeaderFullPathName,
    FileHeaderObjStart,
    FileHeaderObjSize,
    FileDataBytes,
    FileCommit,

    End,
}

impl Default for LibraryParseState {
    fn default() -> Self {
        LibraryParseState::LibraryHeaderStart
    }
}

pub fn parse_library(value: &[u8]) -> Result<MetroWerksLibrary, String> {
    let mut header: LibraryHeader = LibraryHeader::default();
    let mut files: Vec<FileObject> = Vec::new();
    let mut curr_file_header: FileHeader = FileHeader::new();
    let mut curr_file_data = <&[u8]>::default();
    let mut remaining_files = 0;

    let mut offset: usize = 28; // We don't use this var until later, so this is okay.

    let mut state: LibraryParseState = LibraryParseState::default();
    while state != LibraryParseState::End {
        state = match state {
            LibraryParseState::LibraryHeaderStart => LibraryParseState::LibraryHeaderMagicWord,
            LibraryParseState::LibraryHeaderMagicWord => {
                let x = util::convert_be_u32(&value[0..4].try_into().unwrap());

                if x != LibraryMagicWord::LibraryMagicWord as u32 {
                    return Err(format!(
                        "Bad Magic Word: Expected: {}, got: {}",
                        LibraryMagicWord::LibraryMagicWord as u32,
                        x
                    ));
                }

                LibraryParseState::LibraryHeaderProc
            }
            LibraryParseState::LibraryHeaderProc => {
                let x = util::convert_be_u32(&value[4..8].try_into().unwrap());

                header = header.proc(LibraryProcessor::from(x));

                LibraryParseState::LibraryHeaderFlags
            }
            LibraryParseState::LibraryHeaderFlags => {
                // This field is not used per the CW11 API documentation (8..12)
                header = header.flags(LibraryHeaderFlags::None);

                LibraryParseState::LibraryHeaderVersion
            }
            LibraryParseState::LibraryHeaderVersion => {
                let x = util::convert_be_u32(&value[12..16].try_into().unwrap());

                header = header.version(x);

                LibraryParseState::LibraryHeaderCodeSize
            }
            LibraryParseState::LibraryHeaderCodeSize => {
                let x = util::convert_be_u32(&value[16..20].try_into().unwrap());

                header = header.code_size(x);

                LibraryParseState::LibraryHeaderDataSize
            }
            LibraryParseState::LibraryHeaderDataSize => {
                let x = util::convert_be_u32(&value[20..24].try_into().unwrap());

                header = header.data_size(x);

                LibraryParseState::LibraryHeaderNumObjFiles
            }
            LibraryParseState::LibraryHeaderNumObjFiles => {
                let x = util::convert_be_u32(&value[24..28].try_into().unwrap());

                header = header.num_object_files(x);
                remaining_files = x;

                // Next stage starts processing the file objects
                LibraryParseState::FileStart
            }
            LibraryParseState::FileStart => {
                curr_file_header = FileHeader::new();
                curr_file_data = <&[u8]>::default();

                LibraryParseState::FileHeaderModDate
            }
            LibraryParseState::FileHeaderModDate => {
                let x = util::convert_be_u32(&value[offset..(offset + 4)].try_into().unwrap());

                curr_file_header = curr_file_header.mod_date(x);
                offset += 4;

                LibraryParseState::FileHeaderFileName
            }
            LibraryParseState::FileHeaderFileName => {
                let x = util::convert_be_u32(&value[offset..(offset + 4)].try_into().unwrap());

                let file_name = CStr::from_bytes_until_nul(&value[x as usize..])
                    .unwrap()
                    .to_owned();

                offset += 4;
                curr_file_header = curr_file_header.filename(file_name);

                LibraryParseState::FileHeaderFullPathName
            }
            LibraryParseState::FileHeaderFullPathName => {
                let x = util::convert_be_u32(&value[offset..(offset + 4)].try_into().unwrap());

                let full_file_path = if x == 0 {
                    CString::new("").unwrap()
                } else {
                    CStr::from_bytes_until_nul(&value[x as usize..])
                        .unwrap()
                        .to_owned()
                };

                offset += 4;
                curr_file_header = curr_file_header.fullpathname(full_file_path);

                LibraryParseState::FileHeaderObjStart
            }
            LibraryParseState::FileHeaderObjStart => {
                let x = util::convert_be_u32(&value[offset..(offset + 4)].try_into().unwrap());

                curr_file_header = curr_file_header.object_start(x);
                offset += 4;

                LibraryParseState::FileHeaderObjSize
            }
            LibraryParseState::FileHeaderObjSize => {
                let x = util::convert_be_u32(&value[offset..(offset + 4)].try_into().unwrap());

                curr_file_header = curr_file_header.object_size(x);
                offset += 4;

                LibraryParseState::FileDataBytes
            }
            LibraryParseState::FileDataBytes => {
                let start: usize = curr_file_header.start();
                //let size: usize = curr_file_header.object_size;
                let end = curr_file_header.end();

                let bytes = &value[start..end];

                curr_file_data = bytes;
                offset += curr_file_header.length();

                LibraryParseState::FileCommit
            }
            LibraryParseState::FileCommit => {
                let file = FileObject::new(curr_file_header.clone(), curr_file_data);
                files.push(file);
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

    Ok(MetroWerksLibrary::new(header, files))
}

impl TryFrom<&[u8]> for MetroWerksLibrary {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        parse_library(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::objects_m68k::Object;

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
            let ob = Object::try_from(raw_file.as_bytes()).unwrap();
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
