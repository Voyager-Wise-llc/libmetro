use bitflags::bitflags;
use core::fmt::Display;
use std::{ffi::CStr, fmt::Debug};

use super::{
    code_m68k::{CodeHunks, Hunk},
    symtable_m68k::SymbolTable,
    util::{self, RawLength},
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

#[derive(Clone)]
pub struct NameEntry {
    id: u32,
    name: String,
}

impl Debug for NameEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NameEntry")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("hash", &util::nametable_hash(&self.name))
            .finish()
    }
}

impl Display for NameEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl NameEntry {
    pub fn new(id: u32, name: &str) -> NameEntry {
        NameEntry {
            id: id,
            name: name.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn hash(&self) -> u16 {
        util::nametable_hash(&self.name)
    }
}

#[repr(i8)]
#[derive(Debug, Clone)]
pub enum BaseRegister {
    Unknown = -1,
    BaseRegA4 = 4,
    BaseRegA5 = 5,
}

impl From<u8> for BaseRegister {
    fn from(value: u8) -> Self {
        match value {
            4 => BaseRegister::BaseRegA4,
            5 => BaseRegister::BaseRegA5,
            _ => BaseRegister::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetrowerksObject {
    /* header */
    flags: ObjectFlags,
    reserved1: u32,        /* Reserved by Metrowerks */
    old_def_version: u32,  /* CFM68k: Version Info: Old Definition */
    old_imp_version: u32,  /* CFM68k: Version Info: Old Implmentation */
    current_version: u32,  /* CFM68k: Version Info: Current Version */
    has_flags: bool,       /* Reserved by Metrowerks. */
    is_pascal: bool,       /* Reserved by Metrowerks. */
    is_fourbyteint: bool,  /* 68K Processor: 4-Byte Ints */
    is_eightdouble: bool,  /* 68K Processor: 8-Byte Doubles */
    is_mc68881: bool,      /* 68K Processor: Floating Point: 68881 */
    basereg: BaseRegister, /* 68K Target: Library Info: A4, A5 relative data */
    reserved3: u8,         /* Reserved by Metrowerks. This field must contain the value 0L. */
    reserved4: u8,         /* Reserved by Metrowerks. This field must contain the value 0L. */

    names: Vec<NameEntry>,
    symtab: SymbolTable,
    hunks: CodeHunks,
}

impl AsRef<[Hunk]> for MetrowerksObject {
    fn as_ref(&self) -> &[Hunk] {
        &self.hunks.as_ref()
    }
}

impl AsMut<Vec<Hunk>> for MetrowerksObject {
    fn as_mut(&mut self) -> &mut Vec<Hunk> {
        self.hunks.as_mut()
    }
}

impl AsRef<[NameEntry]> for MetrowerksObject {
    fn as_ref(&self) -> &[NameEntry] {
        &self.names.as_ref()
    }
}

impl AsMut<Vec<NameEntry>> for MetrowerksObject {
    fn as_mut(&mut self) -> &mut Vec<NameEntry> {
        self.names.as_mut()
    }
}

impl AsRef<SymbolTable> for MetrowerksObject {
    fn as_ref(&self) -> &SymbolTable {
        &self.symtab
    }
}

impl AsMut<SymbolTable> for MetrowerksObject {
    fn as_mut(&mut self) -> &mut SymbolTable {
        &mut self.symtab
    }
}

impl Default for MetrowerksObject {
    fn default() -> Self {
        Self {
            flags: ObjectFlags::empty(),
            reserved1: 0,
            old_def_version: 0,
            old_imp_version: 0,
            current_version: 0,
            has_flags: false,
            is_pascal: false,
            is_fourbyteint: false,
            is_eightdouble: false,
            is_mc68881: false,
            basereg: BaseRegister::Unknown,
            reserved3: 0,
            reserved4: 0,

            names: vec![],
            symtab: SymbolTable::default(),
            hunks: CodeHunks::new(),
        }
    }
}

impl MetrowerksObject {
    pub fn names(&self) -> &[NameEntry] {
        &self.names
    }

    pub fn hunks(&self) -> &CodeHunks {
        &self.hunks
    }

    pub fn symbols(&self) -> &SymbolTable {
        &self.symtab
    }

    pub fn obj_length(&self) -> usize {
        self.hunks.raw_length()
    }

    pub fn symtable_length(&self) -> usize {
        self.symtab.raw_length()
    }

    pub fn reserved1(&self) -> u32 {
        self.reserved1
    }

    pub fn code_size(&self) -> usize {
        self.hunks.code_length()
    }

    pub fn udata_size(&self) -> usize {
        self.hunks.udata_length()
    }

    pub fn idata_size(&self) -> usize {
        self.hunks.idata_length()
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

    pub fn flags(&self) -> ObjectFlags {
        self.flags
    }

    pub fn has_flags(&self) -> bool {
        self.has_flags
    }

    pub fn is_pascal(&self) -> bool {
        self.is_pascal
    }

    pub fn is_fourbyteint(&self) -> bool {
        self.is_fourbyteint
    }

    pub fn is_eightdouble(&self) -> bool {
        self.is_eightdouble
    }

    pub fn is_mc68881(&self) -> bool {
        self.is_mc68881
    }

    pub fn basereg(&self) -> &BaseRegister {
        &self.basereg
    }

    pub fn reserved3(&self) -> u8 {
        self.reserved3
    }

    pub fn reserved4(&self) -> u8 {
        self.reserved4
    }

    pub fn new(hunks: &CodeHunks, symtab: &SymbolTable) -> MetrowerksObject {
        MetrowerksObject {
            flags: ObjectFlags::empty(),
            reserved1: 0,
            old_def_version: 0,
            old_imp_version: 0,
            current_version: 0,
            has_flags: false,
            is_pascal: false,
            is_fourbyteint: false,
            is_eightdouble: false,
            is_mc68881: false,
            basereg: BaseRegister::Unknown,
            reserved3: 0,
            reserved4: 0,
            names: vec![],
            symtab: symtab.to_owned(),
            hunks: hunks.to_owned(),
        }
    }

    pub fn set_eightdouble(&mut self, arg: bool) {
        self.is_eightdouble = arg;
    }

    pub fn set_fourbyteint(&mut self, arg: bool) {
        self.is_fourbyteint = arg;
    }

    pub fn set_basereg(&mut self, base_reg: BaseRegister) {
        self.basereg = base_reg;
    }

    pub fn set_current_version(&mut self, arg: u32) {
        self.current_version = arg;
    }

    pub fn set_old_def_version(&mut self, arg: u32) {
        self.old_def_version = arg;
    }

    pub fn set_old_imp_version(&mut self, arg: u32) {
        self.old_imp_version = arg;
    }

    pub fn set_mc68881(&mut self, arg: bool) {
        self.is_mc68881 = arg;
    }

    pub fn set_has_flags(&mut self, arg: bool) {
        self.has_flags = arg;
    }

    pub fn set_object_flags(&mut self, arg: ObjectFlags) {
        self.flags = arg;
    }
}

impl TryFrom<&[u8]> for MetrowerksObject {
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
        if version != 0 {
            return Err(format!("Version is not 0L, got {}", version));
        }

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

        // TODO: Keep these here for adding verification to the read later
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
        let basereg = BaseRegister::from(value[61]);

        let reserved3 = value[62];
        if reserved3 != 0 {
            return Err(format!("Reserved is not 0L, got: {}", reserved3));
        }

        let reserved4 = value[63];
        if reserved4 != 0 {
            return Err(format!("Reserved4 is not 0L, got: {}", reserved4));
        }

        let name_table = if nametable_offset != 0 {
            let mut names: Vec<NameEntry> = vec![];
            let mut name_bytes = &value[(nametable_offset as usize)..];
            let mut remaining_names = nametable_count - 1;
            let mut name_id = 1;
            while remaining_names > 0 {
                let hash = util::convert_be_u16(&name_bytes[0..2].try_into().unwrap());
                let s =
                    CStr::from_bytes_until_nul(&name_bytes[2..usize::min(257, name_bytes.len())])
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned();
                let end_of_entry = 2 + s.as_bytes().len() + 1;
                name_bytes = &name_bytes[end_of_entry..];

                // Make sure the computed hash matches whats in the file, else thats a problem.
                assert_eq!(hash, util::nametable_hash(&s));

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
        let sym_tab_start = symtab_offset as usize;
        let sym_tab_end = (symtab_offset + symtable_size) as usize;

        let symtab = if sym_tab_start != 0 {
            let symbol_bytes = &value[sym_tab_start..sym_tab_end];

            SymbolTable::try_from(symbol_bytes).unwrap()
        } else {
            SymbolTable::default()
        };

        // Object code processing
        let code_objects = {
            let start: usize = 64;
            let end: usize = (64 + obj_size) as usize;

            let object_bytes = &value[start..end];

            CodeHunks::try_from(object_bytes).unwrap()
        };

        // Final parse checks
        assert_eq!(code_size as usize, code_objects.code_length());
        assert_eq!(idata_size as usize, code_objects.idata_length());
        assert_eq!(udata_size as usize, code_objects.udata_length());

        let mwob = MetrowerksObject {
            flags: flags.unwrap(),
            reserved1: reserved1,
            old_def_version: old_def_version,
            old_imp_version: old_imp_version,
            current_version: current_version,
            has_flags: has_flags != 0,
            is_pascal: is_pascal != 0,
            is_fourbyteint: is_fourbyteint != 0,
            is_eightdouble: is_eightdouble != 0,
            is_mc68881: is_mc68881 != 0,
            basereg: basereg,
            reserved3: reserved3,
            reserved4: reserved4,

            names: name_table,
            symtab: symtab,
            hunks: code_objects,
        };

        Ok(mwob)
    }
}
