use std::borrow::{Borrow, BorrowMut};
use std::fmt::Debug;

use crate::types_m68k::TypeTable;
use crate::util::{convert_be_i32, RawLength};

use super::types_m68k::{DataType, TypeDefinition};

use super::util::{convert_be_u16, convert_be_u32};

#[derive(PartialEq)]
pub enum SymTableMagicWord {
    SymTableMagicWord = 0x53594D48,
}

#[derive(Debug, Clone)]
pub struct StatementLocation {
    offset: i32,
    source_offset: u32,
}

impl StatementLocation {
    pub fn new(offset: i32, source_offset: u32) -> Self {
        Self {
            offset,
            source_offset,
        }
    }

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

#[derive(Debug, Clone)]
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

    pub fn new(
        name_id: u32,
        typ: DataType,
        kind: StorageKind,
        sclass: StorageClass,
        offset: u32,
    ) -> LocalVar {
        Self {
            name_id,
            var_type: typ,
            kind: kind,
            sclass: sclass,
            wher: offset,
        }
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

impl AsRef<[StatementLocation]> for Routine {
    fn as_ref(&self) -> &[StatementLocation] {
        &self.statement_locations
    }
}

impl AsMut<Vec<StatementLocation>> for Routine {
    fn as_mut(&mut self) -> &mut Vec<StatementLocation> {
        &mut self.statement_locations
    }
}

impl AsRef<[LocalVar]> for Routine {
    fn as_ref(&self) -> &[LocalVar] {
        &self.local_vars
    }
}

impl AsMut<Vec<LocalVar>> for Routine {
    fn as_mut(&mut self) -> &mut Vec<LocalVar> {
        &mut self.local_vars
    }
}

impl Routine {
    pub fn new_func() -> Routine {
        let mut s = Self::default();
        s.typ = RoutineType::Function;
        s
    }

    pub fn new_procedure() -> Routine {
        let mut s = Self::default();
        s.typ = RoutineType::Procedure;
        s
    }

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

#[derive(Debug, Clone)]
pub struct SymbolTable {
    unnamed: u32, // CVW: This may be resolvable where 'name_id == 0' in type table entries.
    reserved: [u32; 4],
    routines: Vec<Routine>,
    types: TypeTable,
}

impl RawLength for SymbolTable {
    fn raw_length(&self) -> usize {
        32 + self.routines.iter().map(|x| x.raw_length()).sum::<usize>()
            + self.types().iter().map(|x| x.raw_length()).sum::<usize>()
    }
}

impl AsRef<[Routine]> for SymbolTable {
    fn as_ref(&self) -> &[Routine] {
        &self.routines.as_ref()
    }
}

impl AsMut<Vec<Routine>> for SymbolTable {
    fn as_mut(&mut self) -> &mut Vec<Routine> {
        self.routines.as_mut()
    }
}

impl AsRef<[TypeDefinition]> for SymbolTable {
    fn as_ref(&self) -> &[TypeDefinition] {
        &self.types.as_ref()
    }
}

impl AsMut<Vec<TypeDefinition>> for SymbolTable {
    fn as_mut(&mut self) -> &mut Vec<TypeDefinition> {
        self.types.as_mut()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            unnamed: 0,
            reserved: [0, 0, 0, 0],
            routines: vec![],
            types: TypeTable::default(),
        }
    }
}

impl SymbolTable {
    pub fn routines(&self) -> &[Routine] {
        &self.routines
    }
    pub fn types(&self) -> &[TypeDefinition] {
        &self.types
    }

    pub fn borrow_routines(&self) -> &Vec<Routine> {
        self.routines.borrow()
    }

    pub fn borrow_routines_mut(&mut self) -> &mut Vec<Routine> {
        self.routines.borrow_mut()
    }

    pub fn borrow_types(&self) -> &Vec<TypeDefinition> {
        self.types.borrow()
    }

    pub fn borrow_types_mut(&mut self) -> &mut Vec<TypeDefinition> {
        self.types.borrow_mut()
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

    pub fn reserved(&self) -> [u32; 4] {
        self.reserved
    }

    pub fn num_unnamed(&self) -> u32 {
        self.unnamed
    }
}

impl TryFrom<&[u8]> for SymbolTable {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // Process header
        let magic = convert_be_u32(&value[0..4].try_into().unwrap());

        if magic != SymTableMagicWord::SymTableMagicWord as u32 {
            return Err(format!(
                "Bad magic word, Expected: {}, got: {}",
                SymTableMagicWord::SymTableMagicWord as u32,
                magic
            ));
        }
        let type_offset = convert_be_u32(&value[4..8].try_into().unwrap()) as usize;
        let num_types = convert_be_u32(&value[8..12].try_into().unwrap());
        let num_unnamed = convert_be_u32(&value[12..16].try_into().unwrap());
        let reserved = convert_reserved(&value[16..32].try_into().unwrap());

        // Process Routines
        let routines = if value.len() > 0 {
            let mut routine_bytes = &value[32..];
            let mut rs: Vec<Routine> = vec![];
            while routine_bytes.len() != 0 {
                let r: Routine = Routine::try_from(routine_bytes).unwrap();
                routine_bytes = &routine_bytes[r.raw_length()..];

                rs.push(r);
            }
            rs
        } else {
            vec![]
        };

        // Process Type Table
        let type_table = if type_offset != 0 {
            let tbl = &value[type_offset..];
            TypeTable::try_from((tbl, num_types)).unwrap()
        } else {
            TypeTable::default()
        };

        Ok(SymbolTable {
            unnamed: num_unnamed,
            reserved: reserved,
            routines: routines,
            types: type_table,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::types_m68k::{PascalString, TypeDefinition, TypeTable};

    use super::SymbolTable;

    #[test]
    fn add_type_def_to_symtab() {
        let mut st = SymbolTable {
            unnamed: 0,
            reserved: [0, 0, 0, 0],
            routines: vec![],
            types: TypeTable::default(),
        };

        assert_eq!(st.types().len(), 0);
        {
            let types = st.borrow_types_mut();

            types.push(TypeDefinition::new(
                crate::types_m68k::OtherDataType::Undefined,
                1234,
            ));
        }

        assert_eq!(st.types().len(), 1);

        {
            let types = st.borrow_types_mut();
            types.push(TypeDefinition::new(
                crate::types_m68k::OtherDataType::TypePascalString(PascalString::new(32, 1)),
                1235,
            ));
        }
        assert_eq!(st.types().len(), 2);
    }
}
