use std::{ops::Range, slice::Iter};

use crate::util::RawLength;

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

        Pointer {
            number: num,
            typ: DataType::from(typ),
        }
    }
}

impl Pointer {
    pub fn number(&self) -> u16 {
        self.number
    }

    pub fn data_type(&self) -> &DataType {
        &self.typ
    }
}
impl RawLength for Pointer {
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

        Array {
            size: size,
            esize: esize,
            typ: DataType::from(typ),
        }
    }
}

impl Array {
    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn esize(&self) -> u32 {
        self.esize
    }

    pub fn data_type(&self) -> &DataType {
        &self.typ
    }
}

impl RawLength for Array {
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
    pub fn data_type(&self) -> &DataType {
        &self.typ
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }
}

impl RawLength for StructMember {
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
            let m = StructMember {
                name_id: name,
                typ: DataType::from(typ),
                offset: offset,
            };
            members.push(m);

            data = &data[12..]
        }

        Struct {
            name_id: name,
            size: size,
            members: members,
        }
    }
}

impl Struct {
    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn members(&self) -> &[StructMember] {
        &self.members
    }

    pub fn member_iter(&self) -> Iter<StructMember> {
        self.members.iter()
    }
}

impl RawLength for Struct {
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
    pub fn value(&self) -> u32 {
        self.value
    }
}

impl RawLength for EnumMember {
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
    pub fn members(&self) -> &[EnumMember] {
        &self.members
    }

    pub fn member_iter(&self) -> Iter<EnumMember> {
        self.members.iter()
    }

    pub fn data_type(&self) -> &DataType {
        &self.typ
    }
}

impl RawLength for Enum {
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
            let m = EnumMember {
                name_id: name,
                value: value,
            };
            members.push(m);

            data = &data[8..]
        }

        let typ: BasicDataType = match DataType::from(baseid as u32) {
            DataType::BasicDataType(x) => x,
            _ => return Err(format!("Bad Type for Enum, got: {}", baseid)),
        };

        Ok(Enum {
            name_id: name,
            typ: DataType::BasicDataType(typ),
            members: members,
        })
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

        PascalArray {
            packed: packed != 0,
            size: size,
            iid: iid,
            eid: DataType::from(eid),
            name_id: name,
        }
    }
}

impl PascalArray {
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
}

impl RawLength for PascalArray {
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

        Self {
            name_id: name,
            typ: DataType::from(base),
            size: size,
            lower: lbound,
            upper: hbound,
        }
    }
}

impl PascalRange {
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
}

impl RawLength for PascalRange {
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

        PascalSet {
            name_id: name,
            base: DataType::from(base),
            size: size,
        }
    }
}

impl PascalSet {
    pub fn base(&self) -> &DataType {
        &self.base
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }
}

impl RawLength for PascalSet {
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

        PascalEnum {
            name_id: name,
            members: members,
        }
    }
}

impl PascalEnum {
    pub fn members_iter(&self) -> Iter<u32> {
        self.members.iter()
    }

    pub fn members(&self) -> &[u32] {
        &self.members
    }
}

impl RawLength for PascalEnum {
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

        PascalString {
            name_id: name,
            size: size,
        }
    }
}

impl PascalString {
    pub fn size(&self) -> u32 {
        self.size
    }
}

impl RawLength for PascalString {
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

impl RawLength for OtherDataType {
    fn raw_length(&self) -> usize {
        match self {
            OtherDataType::Undefined => 0,
            OtherDataType::TypePointer(p) => p.raw_length(),
            OtherDataType::TypeArray(a) => a.raw_length(),
            OtherDataType::TypeStruct(s) => s.raw_length(),
            OtherDataType::TypeEnum(e) => e.raw_length(),
            OtherDataType::TypePascalArray(pa) => pa.raw_length(),
            OtherDataType::TypePascalRange(pr) => pr.raw_length(),
            OtherDataType::TypePascalSet(ps) => ps.raw_length(),
            OtherDataType::TypePascalEnum(pe) => pe.raw_length(),
            OtherDataType::TypePascalString(ps) => ps.raw_length(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeDefinition {
    typ: OtherDataType,
    id: u32,
}

impl RawLength for TypeDefinition {
    fn raw_length(&self) -> usize {
        2 + self.typ.raw_length()
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

impl TryFrom<(u16, u32)> for TypeParseState {
    type Error = &'static str;

    fn try_from(value: (u16, u32)) -> Result<Self, Self::Error> {
        match value.0 {
            x if x == RawOtherDataType::LOCTYPE_POINTER as u16 => {
                Ok(TypeParseState::ParsePointer(value.1))
            }
            x if x == RawOtherDataType::LOCTYPE_ARRAY as u16 => {
                Ok(TypeParseState::ParseArray(value.1))
            }
            x if x == RawOtherDataType::LOCTYPE_STRUCT as u16 => {
                Ok(TypeParseState::ParseStruct(value.1))
            }
            x if x == RawOtherDataType::LOCTYPE_ENUM as u16 => {
                Ok(TypeParseState::ParseEnum(value.1))
            }
            x if x == RawOtherDataType::LOCTYPE_PARRAY as u16 => {
                Ok(TypeParseState::ParsePascalArray(value.1))
            }
            x if x == RawOtherDataType::LOCTYPE_RANGE as u16 => {
                Ok(TypeParseState::ParseRange(value.1))
            }
            x if x == RawOtherDataType::LOCTYPE_SET as u16 => Ok(TypeParseState::ParseSet(value.1)),
            x if x == RawOtherDataType::LOCTYPE_PENUM as u16 => {
                Ok(TypeParseState::ParsePascalEnum(value.1))
            }
            x if x == RawOtherDataType::LOCTYPE_PSTRING as u16 => {
                Ok(TypeParseState::ParsePascalString(value.1))
            }
            _ => Err("Bad branch select for type"),
        }
    }
}

#[derive(Debug, Clone)]
enum TypeParseState {
    ParseTag,

    ParsePointer(u32),
    ParseArray(u32),
    ParseStruct(u32),
    ParseEnum(u32),
    ParsePascalArray(u32),
    ParseRange(u32),
    ParseSet(u32),
    ParsePascalEnum(u32),
    ParsePascalString(u32),
    CommitType(u32, OtherDataType),

    End,
}

impl Default for TypeParseState {
    fn default() -> Self {
        TypeParseState::ParseTag
    }
}

impl PartialEq for TypeParseState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::CommitType(_, _), Self::CommitType(_, _)) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TypeTable {
    table: Vec<TypeDefinition>,
}

impl RawLength for TypeTable {
    fn raw_length(&self) -> usize {
        (2 * self.table.len()) + self.table.iter().map(|x| x.raw_length()).sum::<usize>()
    }
}

impl TryFrom<(&[u8], u32)> for TypeTable {
    type Error = String;

    fn try_from(value: (&[u8], u32)) -> Result<Self, Self::Error> {
        parse_types(value.0, value.1)
    }
}

impl TypeTable {
    pub fn types(&self) -> &[TypeDefinition] {
        &self.table
    }
}

fn parse_types(value: &[u8], num_types: u32) -> Result<TypeTable, String> {
    if num_types == 0 {
        return Ok(TypeTable { table: vec![] });
    }
    let mut data: &[u8] = value;

    let mut types: Vec<TypeDefinition> = vec![];
    let mut remaining_types = num_types;

    let mut state: TypeParseState = TypeParseState::default();
    while state != TypeParseState::End {
        state = match state {
            TypeParseState::ParseTag => {
                let tag = convert_be_u16(&data[0..2].try_into().unwrap());
                let id = convert_be_u32(&data[2..6].try_into().unwrap());

                data = &data[6..];
                TypeParseState::try_from((tag, id)).unwrap() // Jump to the proper processing state
            }

            TypeParseState::ParsePointer(id) => {
                TypeParseState::CommitType(id, OtherDataType::TypePointer(Pointer::from(data)))
            }
            TypeParseState::ParseArray(id) => {
                TypeParseState::CommitType(id, OtherDataType::TypeArray(Array::from(data)))
            }
            TypeParseState::ParseStruct(id) => {
                TypeParseState::CommitType(id, OtherDataType::TypeStruct(Struct::from(data)))
            }
            TypeParseState::ParseEnum(id) => {
                let e = match Enum::try_from(data) {
                    Ok(x) => x,
                    Err(x) => return Err(x),
                };

                TypeParseState::CommitType(id, OtherDataType::TypeEnum(e))
            }
            TypeParseState::ParsePascalArray(id) => TypeParseState::CommitType(
                id,
                OtherDataType::TypePascalArray(PascalArray::from(data)),
            ),
            TypeParseState::ParseRange(id) => TypeParseState::CommitType(
                id,
                OtherDataType::TypePascalRange(PascalRange::from(data)),
            ),
            TypeParseState::ParseSet(id) => {
                TypeParseState::CommitType(id, OtherDataType::TypePascalSet(PascalSet::from(data)))
            }
            TypeParseState::ParsePascalEnum(id) => TypeParseState::CommitType(
                id,
                OtherDataType::TypePascalEnum(PascalEnum::from(data)),
            ),
            TypeParseState::ParsePascalString(id) => TypeParseState::CommitType(
                id,
                OtherDataType::TypePascalString(PascalString::from(data)),
            ),

            TypeParseState::CommitType(id, typ) => {
                data = &data[typ.raw_length()..];

                types.push(TypeDefinition { typ: typ, id: id });
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
