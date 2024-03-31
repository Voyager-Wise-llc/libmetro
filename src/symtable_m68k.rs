use std::slice::Iter;

use super::types_m68k::{DataType, TypeDefinition, TypeTable, TypeTableInput};

use super::util::{convert_be_u16, convert_be_u32, NameIdFromObject};

#[derive(PartialEq)]
pub enum SymTableMagicWord {
    SymTableMagicWord = 0x53594D48,
}

pub struct SymbolTableHeader {
    type_offset: u32,
    num_types: u32,
    unnamed: u32,
    reserved: [u32; 4],
}

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
enum ParseState {
    SymTabHeaderStart,
    SymTabHeaderMagicWord,
    SymTabHeaderTypeOffset,
    SymTabHeaderTypes,
    SymTabHeaderUnnamed,
    SymTabHeaderReserved,

    ProcessTypeTableStart,

    ProcessRoutineTableStart,
    ProcessRoutineStart,
    ProcessRoutineProcFunc,
    ProcessRoutineStatementList,
    ProcessRoutineLocalVarsStart,
    ProcessRoutineLocalVars,

    End,
}

impl Default for ParseState {
    fn default() -> Self {
        ParseState::SymTabHeaderStart
    }
}

pub struct SymbolTable {
    routines: Vec<Routine>,
    types: TypeTable,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            routines: vec![],
            types: TypeTable::default(),
        }
    }
}

impl SymbolTable {
    fn new(routines: Vec<Routine>, type_table: TypeTable) -> Self {
        Self {
            routines: routines,
            types: type_table,
        }
    }

    pub fn routines(&self) -> &[Routine] {
        &self.routines
    }

    pub fn types(&self) -> &TypeTable {
        &self.types
    }

    pub fn type_iter(&self) -> Iter<TypeDefinition> {
        self.types.type_iter()
    }
}

#[derive(Debug, Clone)]
pub struct StatementLocation {
    offset: u32,
    source_offset: u32,
}

impl StatementLocation {
    pub fn is_end_of_list(&self) -> bool {
        self.offset == 0xFFFFFFFF
    }

    pub fn obj_offset(&self) -> u32 {
        self.offset
    }

    pub fn sourcecode_offset(&self) -> u32 {
        self.source_offset
    }
}

impl From<[u8; 8]> for StatementLocation {
    fn from(value: [u8; 8]) -> Self {
        let offset = convert_be_u32(&value[0..4].try_into().unwrap());
        let source_offset = convert_be_u32(&value[4..8].try_into().unwrap());

        Self {
            offset: offset,
            source_offset: source_offset,
        }
    }
}

impl From<&[u8]> for StatementLocation {
    fn from(value: &[u8]) -> Self {
        let offset = convert_be_u32(&value[0..4].try_into().unwrap());
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

impl Default for Routine {
    fn default() -> Self {
        Self {
            typ: RoutineType::Unknown,
            statement_locations: vec![],
            local_vars: vec![],
        }
    }
}

impl Routine {
    pub fn typ(self, typ: RoutineType) -> Self {
        Self {
            typ: typ,
            statement_locations: self.statement_locations,
            local_vars: self.local_vars,
        }
    }

    pub fn add_statement_location(&mut self, statement: StatementLocation) {
        self.statement_locations.push(statement);
    }

    pub fn add_local_var(&mut self, local_var: LocalVar) {
        self.local_vars.push(local_var);
    }

    pub fn is_procedure(&self) -> bool {
        self.typ == RoutineType::Procedure
    }

    pub fn is_function(&self) -> bool {
        self.typ == RoutineType::Function
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
    let mut current_routine = Routine::default();
    let mut remaining_local_vars = 0;

    let mut type_table: TypeTable = TypeTable::default();

    let mut state: ParseState = ParseState::default();
    while state != ParseState::End {
        state = match state {
            ParseState::SymTabHeaderStart => ParseState::SymTabHeaderMagicWord,
            ParseState::SymTabHeaderMagicWord => {
                let x = convert_be_u32(&value[0..4].try_into().unwrap());

                if x != SymTableMagicWord::SymTableMagicWord as u32 {
                    return Err(format!(
                        "Bad magic word, Expected: {}, got: {}",
                        SymTableMagicWord::SymTableMagicWord as u32,
                        x
                    ));
                }

                ParseState::SymTabHeaderTypeOffset
            }

            /* Type Table */
            ParseState::SymTabHeaderTypeOffset => {
                let x = convert_be_u32(&value[4..8].try_into().unwrap());

                header = header.type_offset(x);

                if x != 0 {
                    ParseState::SymTabHeaderTypes
                } else {
                    // No Types defined, skip processing types
                    ParseState::SymTabHeaderReserved
                }
            }
            ParseState::SymTabHeaderTypes => {
                let x = convert_be_u32(&value[8..12].try_into().unwrap());

                header = header.num_types(x);

                ParseState::SymTabHeaderUnnamed
            }
            ParseState::SymTabHeaderUnnamed => {
                let x = convert_be_u32(&value[12..16].try_into().unwrap());

                header = header.unnamed(x);

                if header.type_table_start() != 0 {
                    ParseState::ProcessTypeTableStart
                } else {
                    // No Types defined, skip processing types
                    ParseState::SymTabHeaderReserved
                }
            }

            /* Type Table: Process */
            ParseState::ProcessTypeTableStart => {
                let start = header.type_table_start();
                let tbl = &value[start..];
                let num_types = header.type_table_count();

                type_table = TypeTable::try_from(TypeTableInput::new(tbl, num_types)).unwrap();

                ParseState::SymTabHeaderReserved
            }

            /* MetroWerks reserved  */
            ParseState::SymTabHeaderReserved => {
                let x = convert_reserved(&value[16..32].try_into().unwrap());

                header = header.reserved(x);

                ParseState::ProcessRoutineTableStart
            }

            /* Routine Table */
            ParseState::ProcessRoutineTableStart => {
                if value.len() > 32 {
                    routine_bytes = &value[32..];
                    ParseState::ProcessRoutineStart
                } else {
                    ParseState::End
                }
            }
            ParseState::ProcessRoutineStart => {
                current_routine = Routine::default();

                if routine_bytes.len() != 0 {
                    ParseState::ProcessRoutineProcFunc
                } else {
                    ParseState::End
                }
            }
            ParseState::ProcessRoutineProcFunc => {
                let x = convert_be_u16(&routine_bytes[0..2].try_into().unwrap());
                current_routine = current_routine.typ(match x {
                    // TODO: Move this
                    x if x == RoutineType::Procedure as u16 => RoutineType::Procedure,
                    x if x == RoutineType::Function as u16 => RoutineType::Function,
                    _ => {
                        return Err(format!("Bad Routine Type: got {}", x));
                    }
                });

                routine_bytes = &routine_bytes[2..];

                ParseState::ProcessRoutineStatementList
            }

            /* Routine Table: Statement List */
            ParseState::ProcessRoutineStatementList => {
                let x = &routine_bytes[0..8];

                let statement_loc = StatementLocation::from(x);

                let eol = statement_loc.is_end_of_list();
                current_routine.add_statement_location(statement_loc);

                routine_bytes = &routine_bytes[8..];

                if eol {
                    ParseState::ProcessRoutineLocalVarsStart
                } else {
                    ParseState::ProcessRoutineStatementList
                }
            }

            /* Routine Table: Local Vars */
            ParseState::ProcessRoutineLocalVarsStart => {
                remaining_local_vars = convert_be_u16(&routine_bytes[0..2].try_into().unwrap());

                if remaining_local_vars != 0 {
                    routine_bytes = &routine_bytes[2..];
                    ParseState::ProcessRoutineLocalVars
                } else {
                    ParseState::ProcessRoutineStart
                }
            }
            ParseState::ProcessRoutineLocalVars => {
                let x = &routine_bytes[0..14];

                let local = LocalVar::from(x);
                current_routine.add_local_var(local);

                routine_bytes = &routine_bytes[14..];

                remaining_local_vars -= 1;
                if remaining_local_vars != 0 {
                    ParseState::ProcessRoutineLocalVars
                } else {
                    routines.push(current_routine.clone());
                    ParseState::ProcessRoutineStart
                }
            }
            _ => {
                todo!()
            }
        }
    }

    Ok(SymbolTable::new(routines, type_table))
}

impl TryFrom<&[u8]> for SymbolTable {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        parse_symtab(value)
    }
}
