use bitflags::bitflags;
use core::fmt::Display;
use std::{ffi::CStr, slice::Iter};

use crate::util::RawLength;

use super::{
    code_m68k::{CodeHunks, Hunk},
    symtable_m68k::SymbolTable,
    util,
};

#[derive(PartialEq)]
pub enum ObjectMagicWord {
    ObjectMagicWord = 0xfeedbead,
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

#[derive(Debug, Clone)]
pub struct NameEntry {
    id: u32,
    name: String,
}

impl Display for NameEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl NameEntry {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Clone)]
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

impl TryFrom<&[u8]> for ObjectHeader {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let magic = util::convert_be_u32(&value[0..4].try_into().unwrap());

        if magic != ObjectMagicWord::ObjectMagicWord as u32 {
            return Err(format!(
                "Bad magic word, Expected: {}, got: {}",
                ObjectMagicWord::ObjectMagicWord as u32,
                magic
            ));
        }

        let version = util::convert_be_u16(&value[4..6].try_into().unwrap());
        let flags = ObjectFlags::from_bits(util::convert_be_u16(&value[6..8].try_into().unwrap()));
        let obj_size = util::convert_be_u32(&value[8..12].try_into().unwrap());
        let nametable_offset = util::convert_be_u32(&value[12..16].try_into().unwrap());
        let nametable_count = util::convert_be_u32(&value[16..20].try_into().unwrap());
        let symtab_offset = util::convert_be_u32(&value[20..24].try_into().unwrap());
        let symtable_size = util::convert_be_u32(&value[24..28].try_into().unwrap());
        let reserved1 = util::convert_be_u32(&value[28..32].try_into().unwrap());

        if reserved1 != 0 {
            return Err(format!("Reserved1 is not 0L, got: {}", reserved1));
        }

        let code_size = util::convert_be_u32(&value[32..36].try_into().unwrap());
        let udata_size = util::convert_be_u32(&value[36..40].try_into().unwrap());
        let idata_size = util::convert_be_u32(&value[40..44].try_into().unwrap());

        let old_def_version = util::convert_be_u32(&value[44..48].try_into().unwrap());
        let old_imp_version = util::convert_be_u32(&value[48..52].try_into().unwrap());
        let current_version = util::convert_be_u32(&value[52..56].try_into().unwrap());

        let has_flags = value[56];
        let is_pascal = value[57];
        let is_fourbyteint = value[58];
        let is_eightdouble = value[59];
        let is_mc68881 = value[60];
        let basereg = value[61];

        let reserved3 = value[62];
        if reserved3 != 0 {
            return Err(format!("Reserved is not 0L, got: {}", reserved3));
        }

        let reserved4 = value[63];
        if reserved4 != 0 {
            return Err(format!("Reserved4 is not 0L, got: {}", reserved4));
        }

        Ok(ObjectHeader {
            version: version,
            flags: flags.unwrap(),
            obj_size: obj_size,
            nametable_offset: nametable_offset,
            nametable_names: nametable_count - 1,
            symtable_offset: symtab_offset,
            symtable_size: symtable_size,
            reserved1: reserved1,
            code_size: code_size,
            udata_size: udata_size,
            idata_size: idata_size,
            old_def_version: old_def_version,
            old_imp_version: old_imp_version,
            current_version: current_version,
            has_flags: has_flags,
            is_pascal: is_pascal,
            is_fourbyteint: is_fourbyteint,
            is_eightdouble: is_eightdouble,
            is_mc68881: is_mc68881,
            basereg: basereg,
            reserved3: reserved3,
            reserved4: reserved4,
        })
    }
}

impl RawLength for ObjectHeader {
    fn raw_length(&self) -> usize {
        64
    }
}

impl ObjectHeader {
    pub fn obj_start(&self) -> usize {
        64
    }

    pub fn obj_length(&self) -> usize {
        self.obj_size as usize
    }

    pub fn obj_end(&self) -> usize {
        self.obj_start() + self.obj_length()
    }

    pub fn symtable_start(&self) -> usize {
        self.symtable_offset as usize
    }

    pub fn symtable_length(&self) -> usize {
        self.symtable_size as usize
    }

    pub fn symtable_end(&self) -> usize {
        self.symtable_start() + self.symtable_length()
    }

    pub fn nametable_start(&self) -> usize {
        self.nametable_offset as usize
    }

    pub fn nametable_count(&self) -> usize {
        self.nametable_names as usize
    }

    pub fn reserved1(&self) -> u32 {
        self.reserved1
    }

    pub fn code_size(&self) -> u32 {
        self.code_size
    }

    pub fn udata_size(&self) -> u32 {
        self.udata_size
    }

    pub fn idata_size(&self) -> u32 {
        self.idata_size
    }

    pub fn old_def_version(&self) -> u32 {
        self.old_def_version
    }

    pub fn old_imp_version(&self) -> u32 {
        self.old_imp_version
    }

    pub fn current_version(&self) -> u32 {
        self.current_version
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn flags(&self) -> ObjectFlags {
        self.flags
    }

    pub fn has_flags(&self) -> u8 {
        self.has_flags
    }

    pub fn is_pascal(&self) -> u8 {
        self.is_pascal
    }

    pub fn is_fourbyteint(&self) -> u8 {
        self.is_fourbyteint
    }

    pub fn is_eightdouble(&self) -> u8 {
        self.is_eightdouble
    }

    pub fn is_mc68881(&self) -> u8 {
        self.is_mc68881
    }

    pub fn basereg(&self) -> u8 {
        self.basereg
    }

    pub fn reserved3(&self) -> u8 {
        self.reserved3
    }

    pub fn reserved4(&self) -> u8 {
        self.reserved4
    }
}

#[derive(Debug, Clone)]
pub struct MetrowerksObject {
    header: ObjectHeader,
    names: Vec<NameEntry>,
    symtab: Option<SymbolTable>,
    hunks: CodeHunks,
}

impl MetrowerksObject {
    pub fn names(&self) -> &[NameEntry] {
        &self.names
    }

    pub fn names_iter(&self) -> Iter<NameEntry> {
        self.names.iter()
    }

    pub fn symbols(&self) -> Option<&SymbolTable> {
        self.symtab.as_ref()
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

impl TryFrom<&[u8]> for MetrowerksObject {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let header = ObjectHeader::try_from(value)?;

        let name_table = if header.nametable_start() != 0 {
            let mut names: Vec<NameEntry> = vec![];
            let mut name_bytes = &value[header.nametable_start()..];
            let mut remaining_names = header.nametable_count();
            let mut name_id = 1;
            while remaining_names > 0 {
                let s =
                    CStr::from_bytes_until_nul(&name_bytes[2..usize::min(257, name_bytes.len())])
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned();
                let end_of_entry = 2 + s.as_bytes().len() + 1;
                name_bytes = &name_bytes[end_of_entry..];
                names.push(NameEntry {
                    id: name_id,
                    name: s,
                });

                remaining_names -= 1;
                name_id += 1;
            }
            names
        } else {
            vec![]
        };

        // SymTab Processing
        let sym_tab_start = header.symtable_start();
        let sym_tab_end = header.symtable_end();

        let symtab = if sym_tab_start != 0 {
            let symbol_bytes = &value[sym_tab_start..sym_tab_end];

            Option::Some(SymbolTable::try_from(symbol_bytes).unwrap())
        } else {
            Option::None
        };

        // Object code processing
        let code_objects = {
            let start = header.obj_start();
            let end = header.obj_end();

            let object_bytes = &value[start..end];

            CodeHunks::try_from(object_bytes).unwrap()
        };

        Ok(MetrowerksObject {
            header: header,
            names: name_table,
            symtab: symtab,
            hunks: code_objects,
        })
    }
}
