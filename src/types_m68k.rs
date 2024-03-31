use std::{ops::Range, slice::Iter};

use super::util::{convert_be_u16, convert_be_u32, NameIdFromObject};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataType {
    Undefined(()),
    BasicDataType(BasicDataType),
    Other(u32),
}

#[repr(u16)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BasicDataType {
    BasicTypeVoid = 0,
    BasicTypePstring,
    BasicTypeUlong,
    BasicTypeLong,
    BasicTypeFloat10,
    BasicTypeBoolean, /* Pascal boolean (1 byte) */
    BasicTypeUbyte,
    BasicTypeByte,
    BasicTypeChar,
    BasicTypeWchar,
    BasicTypeUword,
    BasicTypeWord,
    BasicTypeFloat4,
    BasicTypeFloat8,
    BasicTypeFloat12,
    BasicTypeComp,
    BasicTypeCstring,
    BasicTypeAIstring,

    MyBasicTypeVoidPtr = 100,
    MyBasicTypeVoidHdl,
    MyBasicTypeCharPtr,
    MyBasicTypeCharHdl,
    MyBasicTypeUcharPtr,
    MyBasicTypeUcharHdl,
    MyBasicTypeFunc,
    MyBasicTypeStringPtr,
    MyBasicTypePstringPtr, /* Pascal str. pointer */
}

impl From<u32> for DataType {
    fn from(value: u32) -> Self {
        match value {
            x if x == BasicDataType::BasicTypeVoid as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeVoid)
            }
            x if x == BasicDataType::BasicTypePstring as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypePstring)
            }
            x if x == BasicDataType::BasicTypeUlong as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeUlong)
            }
            x if x == BasicDataType::BasicTypeLong as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeLong)
            }
            x if x == BasicDataType::BasicTypeFloat10 as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeFloat10)
            }
            x if x == BasicDataType::BasicTypeBoolean as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeBoolean)
            }
            x if x == BasicDataType::BasicTypeUbyte as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeUbyte)
            }
            x if x == BasicDataType::BasicTypeByte as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeByte)
            }
            x if x == BasicDataType::BasicTypeChar as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeChar)
            }
            x if x == BasicDataType::BasicTypeWchar as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeWchar)
            }
            x if x == BasicDataType::BasicTypeUword as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeUword)
            }
            x if x == BasicDataType::BasicTypeWord as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeWord)
            }
            x if x == BasicDataType::BasicTypeFloat4 as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeFloat4)
            }
            x if x == BasicDataType::BasicTypeFloat8 as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeFloat8)
            }
            x if x == BasicDataType::BasicTypeFloat12 as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeFloat12)
            }
            x if x == BasicDataType::BasicTypeComp as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeComp)
            }
            x if x == BasicDataType::BasicTypeCstring as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeCstring)
            }
            x if x == BasicDataType::BasicTypeAIstring as u32 => {
                DataType::BasicDataType(BasicDataType::BasicTypeAIstring)
            }
            x if x == BasicDataType::MyBasicTypeVoidPtr as u32 => {
                DataType::BasicDataType(BasicDataType::MyBasicTypeVoidPtr)
            }
            x if x == BasicDataType::MyBasicTypeVoidHdl as u32 => {
                DataType::BasicDataType(BasicDataType::MyBasicTypeVoidHdl)
            }
            x if x == BasicDataType::MyBasicTypeCharPtr as u32 => {
                DataType::BasicDataType(BasicDataType::MyBasicTypeCharPtr)
            }
            x if x == BasicDataType::MyBasicTypeCharHdl as u32 => {
                DataType::BasicDataType(BasicDataType::MyBasicTypeCharHdl)
            }
            x if x == BasicDataType::MyBasicTypeUcharPtr as u32 => {
                DataType::BasicDataType(BasicDataType::MyBasicTypeUcharPtr)
            }
            x if x == BasicDataType::MyBasicTypeUcharHdl as u32 => {
                DataType::BasicDataType(BasicDataType::MyBasicTypeUcharHdl)
            }
            x if x == BasicDataType::MyBasicTypeFunc as u32 => {
                DataType::BasicDataType(BasicDataType::MyBasicTypeFunc)
            }
            x if x == BasicDataType::MyBasicTypeStringPtr as u32 => {
                DataType::BasicDataType(BasicDataType::MyBasicTypeStringPtr)
            }
            x if x == BasicDataType::MyBasicTypePstringPtr as u32 => {
                DataType::BasicDataType(BasicDataType::MyBasicTypePstringPtr)
            }
            _ => DataType::Other(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pointer {
    number: u16,
    typ: DataType,
}

impl From<&[u8]> for Pointer {
    fn from(value: &[u8]) -> Self {
        let num = convert_be_u16(&value[0..2].try_into().unwrap());
        let typ = convert_be_u32(&value[2..6].try_into().unwrap());

        Pointer::new(num, DataType::from(typ))
    }
}

impl Pointer {
    fn new(num: u16, typ: DataType) -> Self {
        Self {
            number: num,
            typ: typ,
        }
    }

    pub fn number(&self) -> u16 {
        self.number
    }

    pub fn data_type(&self) -> &DataType {
        &self.typ
    }

    fn raw_length(&self) -> usize {
        6
    }
}

#[derive(Debug, Clone)]
pub struct Array {
    size: u32,
    esize: u32,
    typ: DataType,
}

impl From<&[u8]> for Array {
    fn from(value: &[u8]) -> Self {
        let size = convert_be_u32(&value[0..4].try_into().unwrap());
        let esize = convert_be_u32(&value[4..8].try_into().unwrap());
        let typ = convert_be_u32(&value[8..12].try_into().unwrap());

        Array::new(size, esize, DataType::from(typ))
    }
}

impl Array {
    fn new(size: u32, esize: u32, typ: DataType) -> Self {
        Self {
            size: size,
            esize: esize,
            typ: typ,
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn esize(&self) -> u32 {
        self.esize
    }

    pub fn data_type(&self) -> &DataType {
        &self.typ
    }

    fn raw_length(&self) -> usize {
        12
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct StructMember {
    name_id: u32,
    typ: DataType,
    offset: u32,
}
impl StructMember {
    fn new(name: u32, typ: DataType, offset: u32) -> Self {
        Self {
            name_id: name,
            typ: typ,
            offset: offset,
        }
    }

    pub fn data_type(&self) -> &DataType {
        &self.typ
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    fn raw_length(&self) -> usize {
        12
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct Struct {
    name_id: u32,
    size: u32,
    members: Vec<StructMember>,
}

impl From<&[u8]> for Struct {
    fn from(value: &[u8]) -> Self {
        let mut data = value;

        let name = convert_be_u32(&data[0..4].try_into().unwrap());
        let size = convert_be_u32(&data[4..8].try_into().unwrap());
        let num_members = convert_be_u16(&data[8..10].try_into().unwrap());
        data = &data[10..];

        let mut members: Vec<StructMember> = vec![];
        for _idx in 0..num_members {
            let name = convert_be_u32(&data[0..4].try_into().unwrap());
            let typ = convert_be_u32(&data[4..8].try_into().unwrap());
            let offset = convert_be_u32(&data[8..12].try_into().unwrap());
            let m = StructMember::new(name, DataType::from(typ), offset);
            members.push(m);

            data = &data[12..]
        }

        Struct::new(name, size, members)
    }
}

impl Struct {
    fn new(name: u32, size: u32, members: Vec<StructMember>) -> Struct {
        Self {
            name_id: name,
            size: size,
            members: members,
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn members(&self) -> &[StructMember] {
        &self.members
    }

    pub fn member_iter(&self) -> Iter<StructMember> {
        self.members.iter()
    }

    fn raw_length(&self) -> usize {
        10 + self.members.iter().map(|x| x.raw_length()).sum::<usize>()
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct EnumMember {
    name_id: u32,
    value: u32,
}
impl EnumMember {
    fn new(name: u32, value: u32) -> EnumMember {
        Self {
            name_id: name,
            value: value,
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    fn raw_length(&self) -> usize {
        8
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct Enum {
    name_id: u32,
    typ: DataType,
    members: Vec<EnumMember>,
}
impl Enum {
    fn new(name: u32, typ: BasicDataType, members: Vec<EnumMember>) -> Enum {
        Self {
            name_id: name,
            typ: DataType::BasicDataType(typ),
            members: members,
        }
    }

    pub fn members(&self) -> &[EnumMember] {
        &self.members
    }

    pub fn member_iter(&self) -> Iter<EnumMember> {
        self.members.iter()
    }

    pub fn data_type(&self) -> &DataType {
        &self.typ
    }

    fn raw_length(&self) -> usize {
        8 + self.members.iter().map(|x| x.raw_length()).sum::<usize>()
    }
}

impl TryFrom<&[u8]> for Enum {
    type Error = String;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut data = value;

        let name = convert_be_u32(&data[0..4].try_into().unwrap());
        let baseid = convert_be_u16(&data[4..6].try_into().unwrap());
        let num_members = convert_be_u16(&data[6..8].try_into().unwrap());
        data = &data[8..];

        let mut members: Vec<EnumMember> = vec![];
        for _idx in 0..num_members {
            let name = convert_be_u32(&data[0..4].try_into().unwrap());
            let value = convert_be_u32(&data[4..8].try_into().unwrap());
            let m = EnumMember::new(name, value);
            members.push(m);

            data = &data[8..]
        }

        let typ: BasicDataType = match DataType::from(baseid as u32) {
            DataType::BasicDataType(x) => x,
            _ => return Err(format!("Bad Type for Enum, got: {}", baseid)),
        };

        Ok(Enum::new(name, typ, members))
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct PascalArray {
    packed: bool,
    size: u32,
    iid: u32,
    eid: DataType,
    name_id: u32,
}

impl From<&[u8]> for PascalArray {
    fn from(value: &[u8]) -> Self {
        let packed = convert_be_u32(&value[0..4].try_into().unwrap());
        let size = convert_be_u32(&value[4..8].try_into().unwrap());
        let iid = convert_be_u32(&value[8..12].try_into().unwrap());
        let eid = convert_be_u32(&value[12..16].try_into().unwrap());
        let name = convert_be_u32(&value[16..20].try_into().unwrap());

        PascalArray::new(name, packed != 0, size, iid, DataType::from(eid))
    }
}

impl PascalArray {
    fn new(name: u32, packed: bool, size: u32, iid: u32, eid: DataType) -> PascalArray {
        Self {
            packed: packed,
            size: size,
            iid: iid,
            eid: eid,
            name_id: name,
        }
    }

    pub fn is_packed(&self) -> bool {
        self.packed
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn iid(&self) -> u32 {
        self.iid
    }

    pub fn eid(&self) -> &DataType {
        &self.eid
    }

    fn raw_length(&self) -> usize {
        20
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct PascalRange {
    name_id: u32,
    typ: DataType,
    size: u32,
    lower: u32,
    upper: u32,
}

impl From<&[u8]> for PascalRange {
    fn from(value: &[u8]) -> Self {
        let name = convert_be_u32(&value[0..4].try_into().unwrap());
        let base = convert_be_u32(&value[4..8].try_into().unwrap());
        let size = convert_be_u32(&value[8..12].try_into().unwrap());
        let lbound = convert_be_u32(&value[12..16].try_into().unwrap());
        let hbound = convert_be_u32(&value[16..20].try_into().unwrap());

        PascalRange::new(name, DataType::from(base), size, lbound, hbound)
    }
}

impl PascalRange {
    fn new(name: u32, base: DataType, size: u32, lbound: u32, hbound: u32) -> Self {
        Self {
            name_id: name,
            typ: base,
            size: size,
            lower: lbound,
            upper: hbound,
        }
    }

    pub fn lower(&self) -> u32 {
        self.lower
    }

    pub fn upper(&self) -> u32 {
        self.upper
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn data_type(&self) -> &DataType {
        &self.typ
    }

    fn raw_length(&self) -> usize {
        20
    }
}

impl Into<Range<u32>> for PascalRange {
    fn into(self) -> Range<u32> {
        self.lower..self.upper
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct PascalSet {
    name_id: u32,
    base: DataType,
    size: u32,
}

impl From<&[u8]> for PascalSet {
    fn from(value: &[u8]) -> Self {
        let name = convert_be_u32(&value[0..4].try_into().unwrap());
        let base = convert_be_u32(&value[4..8].try_into().unwrap());
        let size = convert_be_u32(&value[8..12].try_into().unwrap());

        PascalSet::new(name, DataType::from(base), size)
    }
}

impl PascalSet {
    fn new(name: u32, base: DataType, size: u32) -> Self {
        Self {
            name_id: name,
            base: base,
            size: size,
        }
    }

    pub fn base(&self) -> &DataType {
        &self.base
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }

    fn raw_length(&self) -> usize {
        12
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct PascalEnum {
    name_id: u32,
    members: Vec<u32>,
}

impl From<&[u8]> for PascalEnum {
    fn from(value: &[u8]) -> Self {
        let mut data = value;

        let name = convert_be_u32(&data[0..4].try_into().unwrap());
        let num_members = convert_be_u16(&data[4..8].try_into().unwrap());
        data = &data[8..];

        let mut members: Vec<u32> = vec![];
        for _idx in 0..num_members {
            let name = convert_be_u32(&data[0..4].try_into().unwrap());
            members.push(name);

            data = &data[4..]
        }

        PascalEnum::new(name, members)
    }
}

impl PascalEnum {
    fn new(name: u32, members: Vec<u32>) -> PascalEnum {
        Self {
            name_id: name,
            members: members,
        }
    }

    pub fn members_iter(&self) -> Iter<u32> {
        self.members.iter()
    }

    pub fn members(&self) -> &[u32] {
        &self.members
    }

    fn raw_length(&self) -> usize {
        8 + (self.members.len() * 4)
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct PascalString {
    size: u32,
    name_id: u32,
}

impl From<&[u8]> for PascalString {
    fn from(value: &[u8]) -> Self {
        let size = convert_be_u32(&value[0..4].try_into().unwrap());
        let name = convert_be_u32(&value[4..8].try_into().unwrap());

        PascalString::new(name, size)
    }
}

impl PascalString {
    fn new(name: u32, size: u32) -> PascalString {
        Self {
            name_id: name,
            size: size,
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    fn raw_length(&self) -> usize {
        8
    }
}

#[derive(Debug, Clone)]
pub enum OtherDataType {
    Undefined,
    TypePointer(Pointer),
    TypeArray(Array),
    TypeStruct(Struct),
    TypeEnum(Enum),
    TypePascalArray(PascalArray),
    TypePascalRange(PascalRange),
    TypePascalSet(PascalSet),
    TypePascalEnum(PascalEnum),
    TypePascalString(PascalString),
}

#[derive(Debug, Clone)]
pub struct TypeDefinition {
    typ: OtherDataType,
    id: u32,
}

impl Default for TypeDefinition {
    fn default() -> Self {
        Self {
            typ: OtherDataType::Undefined,
            id: 0,
        }
    }
}

impl TypeDefinition {
    pub fn data_type(self, typ: OtherDataType) -> Self {
        Self {
            id: self.id,
            typ: typ,
        }
    }

    pub fn id(self, id: u32) -> Self {
        Self {
            id: id,
            typ: self.typ,
        }
    }
}

#[repr(u16)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
enum RawOtherDataType {
    LOCTYPE_POINTER = 0,
    LOCTYPE_ARRAY,
    LOCTYPE_STRUCT,
    LOCTYPE_ENUM,
    LOCTYPE_PARRAY,
    LOCTYPE_RANGE,
    LOCTYPE_SET,
    LOCTYPE_PENUM,
    LOCTYPE_PSTRING,
}

impl TryFrom<u16> for TypeParseState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == RawOtherDataType::LOCTYPE_POINTER as u16 => Ok(TypeParseState::ParsePointer),
            x if x == RawOtherDataType::LOCTYPE_ARRAY as u16 => Ok(TypeParseState::ParseArray),
            x if x == RawOtherDataType::LOCTYPE_STRUCT as u16 => Ok(TypeParseState::ParseStruct),
            x if x == RawOtherDataType::LOCTYPE_ENUM as u16 => Ok(TypeParseState::ParseEnum),
            x if x == RawOtherDataType::LOCTYPE_PARRAY as u16 => {
                Ok(TypeParseState::ParsePascalArray)
            }
            x if x == RawOtherDataType::LOCTYPE_RANGE as u16 => Ok(TypeParseState::ParseRange),
            x if x == RawOtherDataType::LOCTYPE_SET as u16 => Ok(TypeParseState::ParseSet),
            x if x == RawOtherDataType::LOCTYPE_PENUM as u16 => Ok(TypeParseState::ParsePascalEnum),
            x if x == RawOtherDataType::LOCTYPE_PSTRING as u16 => {
                Ok(TypeParseState::ParsePascalString)
            }
            _ => Err("Bad branch select for type"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TypeParseState {
    Start,

    ParseTag,
    ParseTypeID,

    ParsePointer,
    ParseArray,
    ParseStruct,
    ParseEnum,
    ParsePascalArray,
    ParseRange,
    ParseSet,
    ParsePascalEnum,
    ParsePascalString,
    CommitType,

    End,
}

impl Default for TypeParseState {
    fn default() -> Self {
        TypeParseState::Start
    }
}

#[derive(Debug, Clone)]
pub struct TypeTable {
    table: Vec<TypeDefinition>,
}

impl Default for TypeTable {
    fn default() -> Self {
        Self { table: vec![] }
    }
}

impl TypeTable {
    pub fn types(&self) -> &[TypeDefinition] {
        &self.table
    }

    pub fn type_iter(&self) -> Iter<TypeDefinition> {
        self.table.iter()
    }

    pub fn length(&self) -> usize {
        self.table.len()
    }
}

fn parse_types(value: &[u8], num_types: u32) -> Result<TypeTable, String> {
    let mut data: &[u8] = value;

    let mut types: Vec<TypeDefinition> = vec![];
    let mut remaining_types = num_types;

    let mut current_id = 0;
    let mut current_type: OtherDataType = OtherDataType::Undefined;

    let mut curr_branch: TypeParseState = TypeParseState::End;

    let mut state: TypeParseState = TypeParseState::default();
    while state != TypeParseState::End {
        state = match state {
            TypeParseState::Start => {
                if remaining_types != 0 {
                    TypeParseState::ParseTag
                } else {
                    TypeParseState::End
                }
            }
            TypeParseState::ParseTag => {
                let tag_u16 = convert_be_u16(&data[0..2].try_into().unwrap());
                curr_branch = TypeParseState::try_from(tag_u16).unwrap();

                data = &data[2..];
                TypeParseState::ParseTypeID
            }
            TypeParseState::ParseTypeID => {
                current_id = convert_be_u32(&data[0..4].try_into().unwrap());

                data = &data[4..];
                curr_branch // Jump to the proper processing state
            }

            TypeParseState::ParsePointer => {
                let p = Pointer::from(data);
                data = &data[p.raw_length()..];

                current_type = OtherDataType::TypePointer(p);

                TypeParseState::CommitType
            }
            TypeParseState::ParseArray => {
                let a = Array::from(data);
                data = &data[a.raw_length()..];

                current_type = OtherDataType::TypeArray(a);

                TypeParseState::CommitType
            }
            TypeParseState::ParseStruct => {
                let s = Struct::from(data);
                data = &data[s.raw_length()..];

                current_type = OtherDataType::TypeStruct(s);

                TypeParseState::CommitType
            }
            TypeParseState::ParseEnum => {
                let e = match Enum::try_from(data) {
                    Ok(x) => x,
                    Err(x) => return Err(x),
                };
                data = &data[e.raw_length()..];

                current_type = OtherDataType::TypeEnum(e);

                TypeParseState::CommitType
            }
            TypeParseState::ParsePascalArray => {
                let pa = PascalArray::from(data);
                data = &data[pa.raw_length()..];

                current_type = OtherDataType::TypePascalArray(pa);

                TypeParseState::CommitType
            }
            TypeParseState::ParseRange => {
                let pr = PascalRange::from(data);
                data = &data[pr.raw_length()..];

                current_type = OtherDataType::TypePascalRange(pr);

                TypeParseState::CommitType
            }
            TypeParseState::ParseSet => {
                let ps = PascalSet::from(data);
                data = &data[ps.raw_length()..];

                current_type = OtherDataType::TypePascalSet(ps);

                TypeParseState::CommitType
            }
            TypeParseState::ParsePascalEnum => {
                let pe = PascalEnum::from(data);
                data = &data[pe.raw_length()..];

                current_type = OtherDataType::TypePascalEnum(pe);

                TypeParseState::CommitType
            }
            TypeParseState::ParsePascalString => {
                let ps = PascalString::from(data);
                data = &data[ps.raw_length()..];

                current_type = OtherDataType::TypePascalString(ps);

                TypeParseState::CommitType
            }

            TypeParseState::CommitType => {
                types.push(TypeDefinition {
                    typ: current_type.clone(),
                    id: current_id,
                });
                remaining_types -= 1;

                if remaining_types != 0 {
                    TypeParseState::ParseTag
                } else {
                    TypeParseState::End
                }
            }
            _ => todo!(),
        }
    }
    Ok(TypeTable { table: types })
}

impl TryFrom<(&[u8], u32)> for TypeTable {
    type Error = String;

    fn try_from(value: (&[u8], u32)) -> Result<Self, Self::Error> {
        parse_types(value.0, value.1)
    }
}
