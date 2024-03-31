use std::{
    ffi::{CStr, CString},
    slice::Iter,
};

use bitflags::bitflags;

use super::{
    code_m68k::{CodeHunks, Hunk},
    symtable_m68k::SymbolTable,
    util,
};

#[derive(PartialEq)]
pub enum ObjectMagicWord {
    ObjectMagicWord = 0xfeedbead,
}

impl Default for ObjectMagicWord {
    fn default() -> Self {
        ObjectMagicWord::ObjectMagicWord
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectFlags: u16 {
       const OBJFLAG_CFM = 0x0001;
       const OBJFLAG_WEAKIMPORT = 0x0004;
       const OBJFLAG_INITBEFORE= 0x0008;
       const OBJFLAG_CFMSHAREDLIB = 0x0002;
   }
}

pub struct NameEntry {
    id: u32,
    name: CString,
}

impl Default for NameEntry {
    fn default() -> Self {
        Self {
            id: 0,
            name: CString::default(),
        }
    }
}

impl NameEntry {
    fn new(id: u32, name: CString) -> Self {
        Self { id: id, name: name }
    }

    pub fn name(&self) -> &CString {
        &self.name
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

pub struct ObjectHeader {
    version: u16, /* always OBJ_VERSION */
    flags: ObjectFlags,
    obj_size: u32, /* Object data size */
    nametable_offset: u32,
    nametable_names: u32, /* number of names */
    symtable_offset: u32,
    symtable_size: u32,
    reserved1: u32,       /* Reserved by Metrowerks */
    code_size: u32,       /* Executable code size */
    udata_size: u32,      /* Uninitialized data size */
    idata_size: u32,      /* Initialized data size */
    old_def_version: u32, /* CFM68k flag, For object code that doesn’t define a CFM68K shared library, this field contains 0L */
    old_imp_version: u32, /* CFM68k flag, For object code that doesn’t define a CFM68K shared library, this field contains 0L */
    current_version: u32, /* CFM68k flag, For object code that doesn’t define a CFM68K shared library, this field contains 0L */
    has_flags: u8,        /* Reserved by Metrowerks. */
    is_pascal: u8,        /* Reserved by Metrowerks. */
    is_fourbyteint: u8,   /* Reserved by Metrowerks. */
    is_eightdouble: u8,   /* Reserved by Metrowerks. */
    is_mc68881: u8,       /* Reserved by Metrowerks. */
    basereg: u8,          /* Reserved by Metrowerks. */
    reserved3: u8,        /* Reserved by Metrowerks. This field must contain the value 0L. */
    reserved4: u8,        /* Reserved by Metrowerks. This field must contain the value 0L. */
}

impl Default for ObjectHeader {
    fn default() -> Self {
        Self {
            version: 0,
            flags: ObjectFlags::empty(),
            obj_size: 0,
            nametable_offset: 0,
            nametable_names: 0,
            symtable_offset: 0,
            symtable_size: 0,
            reserved1: 0,
            code_size: 0,
            udata_size: 0,
            idata_size: 0,
            old_def_version: 0,
            old_imp_version: 0,
            current_version: 0,
            has_flags: 0,
            is_pascal: 0,
            is_fourbyteint: 0,
            is_eightdouble: 0,
            is_mc68881: 0,
            basereg: 0,
            reserved3: 0,
            reserved4: 0,
        }
    }
}

impl ObjectHeader {
    pub fn version(self, version: u16) -> Self {
        Self {
            version: version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    pub fn flags(self, flags: ObjectFlags) -> Self {
        Self {
            version: self.version,
            flags: flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    pub fn obj_size(self, obj_size: u32) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    #[inline(always)]
    pub fn obj_start(&self) -> usize {
        64 as usize
    }

    #[inline(always)]
    pub fn obj_length(&self) -> usize {
        self.obj_size as usize
    }

    #[inline(always)]
    pub fn obj_end(&self) -> usize {
        self.obj_start() + self.obj_length()
    }

    pub fn nametable_offset(self, nametable_offset: u32) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    #[inline(always)]
    pub fn nametable_start(&self) -> usize {
        self.nametable_offset as usize
    }

    #[inline(always)]
    pub fn nametable_count(&self) -> usize {
        self.nametable_names as usize
    }

    pub fn num_names(self, nametable_num: u32) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: nametable_num,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    pub fn symtable_offset(self, symtable_offset: u32) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    pub fn symtable_size(self, symtable_size: u32) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    #[inline(always)]
    pub fn symtable_start(&self) -> usize {
        self.symtable_offset as usize
    }

    #[inline(always)]
    pub fn symtable_length(&self) -> usize {
        self.symtable_size as usize
    }

    #[inline(always)]
    pub fn symtable_end(&self) -> usize {
        self.symtable_start() + self.symtable_length()
    }

    pub fn code_size(self, code_size: u32) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    pub fn udata_size(self, udata_size: u32) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    pub fn idata_size(self, idata_size: u32) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    pub fn basereg(self, basereg: u8) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    pub fn has_flags(self, has_flags: u8) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    pub fn is_fourbyteint(self, is_fourbyteint: u8) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    pub fn is_eightdouble(self, is_eightdouble: u8) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    pub fn is_mc68881(self, is_mc68881: u8) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: self.is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }

    pub fn is_pascal(self, is_pascal: u8) -> Self {
        Self {
            version: self.version,
            flags: self.flags,
            obj_size: self.obj_size,
            nametable_offset: self.nametable_offset,
            nametable_names: self.nametable_names,
            symtable_offset: self.symtable_offset,
            symtable_size: self.symtable_size,
            reserved1: self.reserved1,
            code_size: self.code_size,
            udata_size: self.udata_size,
            idata_size: self.idata_size,
            old_def_version: self.old_def_version,
            old_imp_version: self.old_imp_version,
            current_version: self.current_version,
            has_flags: self.has_flags,
            is_pascal: is_pascal,
            is_fourbyteint: self.is_fourbyteint,
            is_eightdouble: self.is_eightdouble,
            is_mc68881: self.is_mc68881,
            basereg: self.basereg,
            reserved3: self.reserved3,
            reserved4: self.reserved4,
        }
    }
}

pub struct MetrowerksObject {
    header: ObjectHeader,
    names: Vec<NameEntry>,
    symtab: SymbolTable,
    hunks: CodeHunks,
}

impl Default for MetrowerksObject {
    fn default() -> Self {
        Self {
            header: ObjectHeader::default(),
            names: vec![],
            symtab: SymbolTable::default(),
            hunks: CodeHunks::default(),
        }
    }
}

impl MetrowerksObject {
    fn new(
        header: ObjectHeader,
        names: Vec<NameEntry>,
        symtab: SymbolTable,
        hunks: CodeHunks,
    ) -> Self {
        Self {
            header: header,
            names: names,
            symtab: symtab,
            hunks: hunks,
        }
    }

    pub fn names(&self) -> &[NameEntry] {
        &self.names
    }

    pub fn names_iter(&self) -> Iter<NameEntry> {
        self.names.iter()
    }

    pub fn symbols(&self) -> &SymbolTable {
        &self.symtab
    }

    pub fn hunk_iter(&self) -> Iter<Hunk> {
        self.hunks.iter()
    }

    pub fn hunks(&self) -> &CodeHunks {
        &self.hunks
    }

    pub fn header(&self) -> &ObjectHeader {
        &self.header
    }
}

#[derive(Debug, PartialEq)]
enum ObjectParseState {
    ObjectHeaderStart,
    ObjectHeaderMagicWord,
    ObjectHeaderVersion,
    ObjectHeaderFlags,
    ObjectHeaderObjectSize,
    ObjectHeaderNameTableOffset,
    ObjectHeaderNameTableCount,
    ObjectHeaderSymTableOffset,
    ObjectHeaderSymTableSize,
    ObjectHeaderReserved1,
    ObjectHeaderCodeSize,
    ObjectHeaderUninitializedDataSize,
    ObjectHeaderInitializedDataSize,
    ObjectHeaderCFM68kOldDefinitionVersion,
    ObjectHeaderCFM68kOldImplmentationVersion,
    ObjectHeaderCFM68kCurrentVersion,
    ObjectHeaderReservedHasFlags,
    ObjectHeaderReservedIsPascal,
    ObjectHeaderReservedIsFourByteInt,
    ObjectHeaderReservedIsEightDouble,
    ObjectHeaderReservedIsMC68881,
    ObjectHeaderReservedBaseReg,
    ObjectHeaderReserved3,
    ObjectHeaderReserved4,

    ProcessNameTable,
    ProcessName,
    ProcessSymbolTable,
    ProcessObjectData,

    End,
}

impl Default for ObjectParseState {
    fn default() -> Self {
        ObjectParseState::ObjectHeaderStart
    }
}

fn parse_object(value: &[u8]) -> Result<MetrowerksObject, String> {
    let mut header: ObjectHeader = ObjectHeader::default();

    let mut remaining_names: usize = 0;

    let mut name_bytes: &[u8] = <&[u8]>::default();
    let mut name_table: Vec<NameEntry> = vec![];
    let mut name_id = 0;

    let mut symbol_table: SymbolTable = SymbolTable::default();

    let mut code_objects: CodeHunks = CodeHunks::default();

    let mut state: ObjectParseState = ObjectParseState::default();
    while state != ObjectParseState::End {
        state = match state {
            ObjectParseState::ObjectHeaderStart => ObjectParseState::ObjectHeaderMagicWord,
            ObjectParseState::ObjectHeaderMagicWord => {
                let x = util::convert_be_u32(&value[0..4].try_into().unwrap());

                if x != ObjectMagicWord::ObjectMagicWord as u32 {
                    return Err(format!(
                        "Bad magic word, Expected: {}, got: {}",
                        ObjectMagicWord::ObjectMagicWord as u32,
                        x
                    ));
                }

                ObjectParseState::ObjectHeaderVersion
            }
            ObjectParseState::ObjectHeaderVersion => {
                let x = util::convert_be_u16(&value[4..6].try_into().unwrap());

                header = header.version(x);

                ObjectParseState::ObjectHeaderFlags
            }
            ObjectParseState::ObjectHeaderFlags => {
                let x = util::convert_be_u16(&value[6..8].try_into().unwrap());

                header = header.flags(ObjectFlags::from_bits(x).unwrap());

                ObjectParseState::ObjectHeaderObjectSize
            }

            /* Object Segment */
            ObjectParseState::ObjectHeaderObjectSize => {
                let x = util::convert_be_u32(&value[8..12].try_into().unwrap());

                header = header.obj_size(x);

                ObjectParseState::ProcessObjectData
            }
            ObjectParseState::ProcessObjectData => {
                let start = header.obj_start();
                let end = header.obj_end();

                let object_bytes = &value[start..end];

                code_objects = CodeHunks::try_from(object_bytes).unwrap();

                ObjectParseState::ObjectHeaderNameTableOffset
            }

            /* Name table */
            ObjectParseState::ObjectHeaderNameTableOffset => {
                let x = util::convert_be_u32(&value[12..16].try_into().unwrap());

                header = header.nametable_offset(x);

                ObjectParseState::ObjectHeaderNameTableCount
            }
            ObjectParseState::ObjectHeaderNameTableCount => {
                let x = util::convert_be_u32(&value[16..20].try_into().unwrap());

                if x != 0 {
                    header = header.num_names(x - 1);
                    ObjectParseState::ProcessNameTable
                } else {
                    ObjectParseState::ObjectHeaderSymTableOffset
                }
            }
            ObjectParseState::ProcessNameTable => {
                let start: usize = header.nametable_start();

                if start != 0 {
                    name_bytes = &value[start..];
                    remaining_names = header.nametable_count();
                    name_id = 1;

                    ObjectParseState::ProcessName
                } else {
                    ObjectParseState::ObjectHeaderSymTableOffset
                }
            }
            ObjectParseState::ProcessName => {
                let s =
                    CStr::from_bytes_until_nul(&name_bytes[2..usize::min(258, name_bytes.len())])
                        .unwrap()
                        .to_owned();

                let end_of_entry = 2 + s.as_bytes().len() + 1;
                name_bytes = &name_bytes[end_of_entry..];
                name_table.push(NameEntry::new(name_id, s));

                remaining_names -= 1;
                name_id += 1;

                if remaining_names != 0 {
                    ObjectParseState::ProcessName
                } else {
                    ObjectParseState::ObjectHeaderSymTableOffset
                }
            }

            /* Symbol Table */
            ObjectParseState::ObjectHeaderSymTableOffset => {
                let x = util::convert_be_u32(&value[20..24].try_into().unwrap());

                header = header.symtable_offset(x);

                ObjectParseState::ObjectHeaderSymTableSize
            }
            ObjectParseState::ObjectHeaderSymTableSize => {
                let x = util::convert_be_u32(&value[24..28].try_into().unwrap());

                header = header.symtable_size(x);

                ObjectParseState::ProcessSymbolTable
            }
            ObjectParseState::ProcessSymbolTable => {
                let start = header.symtable_start();
                let end = header.symtable_end();

                if start != 0 {
                    let symbol_bytes = &value[start..end];

                    symbol_table = SymbolTable::try_from(symbol_bytes).unwrap();
                }

                ObjectParseState::ObjectHeaderReserved1
            }

            /* Metrowerks Reserved Field */
            ObjectParseState::ObjectHeaderReserved1 => {
                let x = util::convert_be_u32(&value[28..32].try_into().unwrap());

                if x != 0 {
                    return Err(format!("{:#?} is not 0L, got: {}", state, x));
                }

                ObjectParseState::ObjectHeaderCodeSize
            }

            /* Code and Data sizes */
            ObjectParseState::ObjectHeaderCodeSize => {
                let x = util::convert_be_u32(&value[32..36].try_into().unwrap());

                header = header.code_size(x);

                ObjectParseState::ObjectHeaderUninitializedDataSize
            }
            ObjectParseState::ObjectHeaderUninitializedDataSize => {
                let x = util::convert_be_u32(&value[36..40].try_into().unwrap());

                header = header.udata_size(x);

                ObjectParseState::ObjectHeaderInitializedDataSize
            }
            ObjectParseState::ObjectHeaderInitializedDataSize => {
                let x = util::convert_be_u32(&value[40..44].try_into().unwrap());

                header = header.idata_size(x);

                ObjectParseState::ObjectHeaderCFM68kOldDefinitionVersion
            }

            /* CFM68K fields: TODO */
            ObjectParseState::ObjectHeaderCFM68kOldDefinitionVersion => {
                let _x = util::convert_be_u32(&value[44..48].try_into().unwrap());

                ObjectParseState::ObjectHeaderCFM68kOldImplmentationVersion
            }
            ObjectParseState::ObjectHeaderCFM68kOldImplmentationVersion => {
                let _x = util::convert_be_u32(&value[48..52].try_into().unwrap());

                ObjectParseState::ObjectHeaderCFM68kCurrentVersion
            }
            ObjectParseState::ObjectHeaderCFM68kCurrentVersion => {
                let _x = util::convert_be_u32(&value[52..56].try_into().unwrap());

                ObjectParseState::ObjectHeaderReservedHasFlags
            }

            /* Metrowerks Reserved Fields */
            ObjectParseState::ObjectHeaderReservedHasFlags => {
                let x = value[56];
                header = header.has_flags(x);

                ObjectParseState::ObjectHeaderReservedIsPascal
            }
            ObjectParseState::ObjectHeaderReservedIsPascal => {
                let x = value[57];
                header = header.is_pascal(x);

                ObjectParseState::ObjectHeaderReservedIsFourByteInt
            }
            ObjectParseState::ObjectHeaderReservedIsFourByteInt => {
                let x = value[58];
                header = header.is_fourbyteint(x);

                ObjectParseState::ObjectHeaderReservedIsEightDouble
            }
            ObjectParseState::ObjectHeaderReservedIsEightDouble => {
                let x = value[59];
                header = header.is_eightdouble(x);

                ObjectParseState::ObjectHeaderReservedIsMC68881
            }
            ObjectParseState::ObjectHeaderReservedIsMC68881 => {
                let x = value[60];
                header = header.is_mc68881(x);

                ObjectParseState::ObjectHeaderReservedBaseReg
            }
            ObjectParseState::ObjectHeaderReservedBaseReg => {
                let x = value[61];
                header = header.basereg(x);

                ObjectParseState::ObjectHeaderReserved3
            }
            ObjectParseState::ObjectHeaderReserved3 => {
                let x = value[62];
                if x != 0 {
                    return Err(format!("{:#?} is not 0L, got: {}", state, x));
                }

                ObjectParseState::ObjectHeaderReserved4
            }
            ObjectParseState::ObjectHeaderReserved4 => {
                let x = value[63];
                if x != 0 {
                    return Err(format!("{:#?} is not 0L, got: {}", state, x));
                }

                ObjectParseState::End
            }
            _ => todo!(),
        }
    }

    Ok(MetrowerksObject::new(
        header,
        name_table,
        symbol_table,
        code_objects,
    ))
}

impl TryFrom<&[u8]> for MetrowerksObject {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        parse_object(value)
    }
}
