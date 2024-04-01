use std::fmt::Debug;
use std::slice::Iter;

use crate::types_m68k::TypeTable;
use crate::util::{convert_be_i32, RawLength};

use super::types_m68k::{DataType, TypeDefinition};

use super::util::{convert_be_u16, convert_be_u32, NameIdFromObject};

#[derive(PartialEq)]
pub enum SymTableMagicWord {
    SymTableMagicWord = 0x53594D48,
}

struct SymbolTableHeader {
    type_offset: u32,
    num_types: u32,
    unnamed: u32,
    reserved: [u32; 4],
}

// TODO: This also sucks and needs refactoring
impl Default for SymbolTableHeader {
    fn default() -> Self {
        Self {
            type_offset: 0,
            num_types: 0,
            unnamed: 0,
            reserved: [0, 0, 0, 0],
        }
    }
}

impl SymbolTableHeader {
    fn type_offset(self, type_offset: u32) -> Self {
        Self {
            type_offset: type_offset,
            num_types: self.num_types,
            unnamed: self.unnamed,
            reserved: self.reserved,
        }
    }
    fn num_types(self, types: u32) -> Self {
        Self {
            type_offset: self.type_offset,
            num_types: types,
            unnamed: self.unnamed,
            reserved: self.reserved,
        }
    }
    pub fn unnamed(self, unnamed: u32) -> Self {
        Self {
            type_offset: self.type_offset,
            num_types: self.num_types,
            unnamed: unnamed,
            reserved: self.reserved,
        }
    }
    pub fn reserved(self, reserved: [u32; 4]) -> Self {
        Self {
            type_offset: self.type_offset,
            num_types: self.num_types,
            unnamed: self.unnamed,
            reserved: reserved,
        }
    }

    #[inline(always)]
    pub fn type_table_start(&self) -> usize {
        self.type_offset as usize
    }

    #[inline(always)]
    pub fn type_table_count(&self) -> u32 {
        self.num_types
    }
}

#[derive(PartialEq)]
enum SymTabParseState {
    SymTabHeaderStart,
    SymTabHeaderMagicWord,
    SymTabHeaderTypeOffset,
    SymTabHeaderTypes,
    SymTabHeaderUnnamed,
    SymTabHeaderReserved,

    ProcessRoutineTableStart,
    ProcessRoutines,

    End,
}

impl Default for SymTabParseState {
    fn default() -> Self {
        SymTabParseState::SymTabHeaderStart
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    routines: Vec<Routine>,
    types: Vec<TypeDefinition>,
}

impl TryFrom<&[u8]> for SymbolTable {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        parse_symtab(value)
    }
}

impl RawLength for SymbolTable {
    fn raw_length(&self) -> usize {
        32 + self.routines.iter().map(|x| x.raw_length()).sum::<usize>()
            + self.types().iter().map(|x| x.raw_length()).sum::<usize>()
    }
}

impl SymbolTable {
    pub fn routines(&self) -> &[Routine] {
        &self.routines
    }

    pub fn routine_iter(&self) -> Iter<Routine> {
        self.routines.iter()
    }

    pub fn types(&self) -> &[TypeDefinition] {
        &self.types
    }

    pub fn type_iter(&self) -> Iter<TypeDefinition> {
        self.types.iter()
    }

    pub fn routine_at_offset(&self, offset: usize) -> &Routine {
        let mut i = 0;
        let mut off = offset;

        // Remove the Symtab header
        off -= 32;

        let mut iter = self.routines.iter();
        while off > 0 {
            let r = iter.next().unwrap();
            off -= r.raw_length();
            i += 1;
        }

        &self.routines[i]
    }
}

#[derive(Debug, Clone)]
pub struct StatementLocation {
    offset: i32,
    source_offset: u32,
}

impl StatementLocation {
    pub fn is_end_of_list(&self) -> bool {
        self.offset == -1
    }

    pub fn obj_offset(&self) -> i32 {
        self.offset
    }

    pub fn sourcecode_offset(&self) -> u32 {
        self.source_offset
    }

    fn raw_length(&self) -> usize {
        8
    }
}

impl From<&[u8]> for StatementLocation {
    fn from(value: &[u8]) -> Self {
        let offset = convert_be_i32(&value[0..4].try_into().unwrap());
        let source_offset = convert_be_u32(&value[4..8].try_into().unwrap());

        Self {
            offset: offset,
            source_offset: source_offset,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StorageKind {
    Local = 0,
    Value,
    Reference,
}

impl TryFrom<u8> for StorageKind {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            x if x == StorageKind::Local as u8 => StorageKind::Local,
            x if x == StorageKind::Value as u8 => StorageKind::Value,
            x if x == StorageKind::Reference as u8 => StorageKind::Reference,
            _ => {
                return Err("Bad Storage Kind");
            }
        })
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StorageClass {
    Register = 0,
    A5,
    A6,
    A7,
}

impl TryFrom<u8> for StorageClass {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            x if x == StorageClass::Register as u8 => StorageClass::Register,
            x if x == StorageClass::A5 as u8 => StorageClass::A5,
            x if x == StorageClass::A6 as u8 => StorageClass::A6,
            x if x == StorageClass::A7 as u8 => StorageClass::A7,

            _ => {
                return Err("Bad Storage Kind");
            }
        })
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct LocalVar {
    name_id: u32,
    var_type: DataType,
    kind: StorageKind,
    sclass: StorageClass,
    wher: u32, // TODO: Integrate this into the sclass
}

impl From<&[u8]> for LocalVar {
    fn from(value: &[u8]) -> Self {
        let name_id = convert_be_u32(value[0..4].try_into().unwrap());
        let var_type = convert_be_u32(value[4..8].try_into().unwrap());
        let kind = StorageKind::try_from(value[8]).unwrap();
        let sclass = StorageClass::try_from(value[9]).unwrap();
        let wher = convert_be_u32(value[10..14].try_into().unwrap());

        Self {
            name_id: name_id,
            var_type: DataType::from(var_type),
            kind: kind,
            sclass: sclass,
            wher: wher,
        }
    }
}

impl LocalVar {
    pub fn var_type(&self) -> &DataType {
        &self.var_type
    }

    pub fn kind(&self) -> StorageKind {
        self.kind
    }

    pub fn storage_class(&self) -> StorageClass {
        self.sclass
    }

    pub fn wher(&self) -> u32 {
        self.wher
    }

    fn raw_length(&self) -> usize {
        14
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoutineType {
    Procedure = 0,
    Function = 1,
    Unknown = 0xffff,
}

#[derive(Debug, Clone)]
pub struct Routine {
    typ: RoutineType,
    statement_locations: Vec<StatementLocation>,
    local_vars: Vec<LocalVar>,
}

impl TryFrom<&[u8]> for Routine {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut data = value;
        let mut statement_locs: Vec<StatementLocation> = vec![];
        let mut local_vars: Vec<LocalVar> = vec![];

        // Get routine type
        let routine_type = convert_be_u16(&data[0..2].try_into().unwrap());
        let typ = match routine_type {
            x if x == RoutineType::Procedure as u16 => RoutineType::Procedure,
            x if x == RoutineType::Function as u16 => RoutineType::Function,
            _ => {
                return Err(format!("Bad Routine Type: got {}", routine_type));
            }
        };

        data = &data[2..];
        let mut eol = false;
        while !eol {
            let statement_loc = StatementLocation::from(data);
            data = &data[statement_loc.raw_length()..];
            eol = statement_loc.is_end_of_list();
            statement_locs.push(statement_loc);
        }

        let mut remaining_local_vars = convert_be_u16(&data[0..2].try_into().unwrap());
        data = &data[2..];

        while remaining_local_vars != 0 {
            let local = LocalVar::from(data);
            data = &data[local.raw_length()..];

            local_vars.push(local);

            remaining_local_vars -= 1;
        }

        Ok(Routine {
            typ: typ,
            statement_locations: statement_locs,
            local_vars: local_vars,
        })
    }
}

impl Routine {
    pub fn statement_locations(&self) -> &[StatementLocation] {
        self.statement_locations.as_slice()
    }

    pub fn local_vars(&self) -> &[LocalVar] {
        self.local_vars.as_slice()
    }

    pub fn is_procedure(&self) -> bool {
        self.typ == RoutineType::Procedure
    }

    pub fn is_function(&self) -> bool {
        self.typ == RoutineType::Function
    }

    fn raw_length(&self) -> usize {
        4 + self
            .statement_locations
            .iter()
            .map(|x| x.raw_length())
            .sum::<usize>()
            + self
                .local_vars
                .iter()
                .map(|x| x.raw_length())
                .sum::<usize>()
    }
}

fn convert_reserved(data: &[u8; 16]) -> [u32; 4] {
    let res: [u32; 4] = unsafe { std::mem::transmute(*data) };
    res.map(|v| u32::from_be(v))
}

fn parse_symtab(value: &[u8]) -> Result<SymbolTable, String> {
    let mut header: SymbolTableHeader = SymbolTableHeader::default();

    let mut routines: Vec<Routine> = vec![];
    let mut routine_bytes: &[u8] = <&[u8]>::default();

    let mut state: SymTabParseState = SymTabParseState::default();
    while state != SymTabParseState::End {
        state = match state {
            SymTabParseState::SymTabHeaderStart => SymTabParseState::SymTabHeaderMagicWord,
            SymTabParseState::SymTabHeaderMagicWord => {
                let x = convert_be_u32(&value[0..4].try_into().unwrap());

                if x != SymTableMagicWord::SymTableMagicWord as u32 {
                    return Err(format!(
                        "Bad magic word, Expected: {}, got: {}",
                        SymTableMagicWord::SymTableMagicWord as u32,
                        x
                    ));
                }

                SymTabParseState::SymTabHeaderTypeOffset
            }

            /* Type Table */
            SymTabParseState::SymTabHeaderTypeOffset => {
                let x = convert_be_u32(&value[4..8].try_into().unwrap());

                header = header.type_offset(x);

                if x != 0 {
                    SymTabParseState::SymTabHeaderTypes
                } else {
                    // No Types defined, skip processing types
                    SymTabParseState::SymTabHeaderReserved
                }
            }
            SymTabParseState::SymTabHeaderTypes => {
                let x = convert_be_u32(&value[8..12].try_into().unwrap());

                header = header.num_types(x);

                SymTabParseState::SymTabHeaderUnnamed
            }
            SymTabParseState::SymTabHeaderUnnamed => {
                let x = convert_be_u32(&value[12..16].try_into().unwrap());

                header = header.unnamed(x);

                SymTabParseState::SymTabHeaderReserved
            }

            /* MetroWerks reserved  */
            SymTabParseState::SymTabHeaderReserved => {
                let x = convert_reserved(&value[16..32].try_into().unwrap());

                header = header.reserved(x);

                SymTabParseState::ProcessRoutineTableStart
            }

            /* Routine Table */
            SymTabParseState::ProcessRoutineTableStart => {
                if value.len() > 32 {
                    routine_bytes = &value[32..];
                    SymTabParseState::ProcessRoutines
                } else {
                    SymTabParseState::End
                }
            }
            SymTabParseState::ProcessRoutines => {
                while routine_bytes.len() != 0 {
                    let r: Routine = Routine::try_from(routine_bytes).unwrap();
                    routine_bytes = &routine_bytes[r.raw_length()..];

                    routines.push(r);
                }

                SymTabParseState::End
            }
            _ => {
                todo!()
            }
        }
    }

    let tt_start = header.type_table_start();

    let type_table = if tt_start != 0 {
        let tbl = &value[tt_start..];
        let num_types = header.type_table_count();

        TypeTable::try_from((tbl, num_types))
            .unwrap()
            .types()
            .to_owned()
    } else {
        vec![]
    };

    Ok(SymbolTable {
        routines: routines,
        types: type_table,
    })
}
