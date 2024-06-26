use std::ops::Deref;

use chrono::{DateTime, Local};

use crate::util::{from_mac_datetime, RawLength};

use super::util::{convert_be_u16, convert_be_u32, NameIdFromObject};

#[derive(Debug, Clone)]
pub struct ReservedHunk {}

#[derive(Debug, Clone)]
pub struct ObjSimpleHunk {}

impl RawLength for ObjSimpleHunk {
    fn raw_length(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ObjCodeFlag {
    None,
    GlobalMultiDef,
    GlobalOverload,
    CFMExport,
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct ObjCodeHunk {
    name_id: u32,
    sym_offset: u32,
    sym_decl_offset: u32,
    special_flag: ObjCodeFlag,
    code: Vec<u8>,
}

impl Deref for ObjCodeHunk {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl RawLength for ObjCodeHunk {
    fn raw_length(&self) -> usize {
        12 + self.code.len()
    }
}

impl ObjCodeHunk {
    pub fn has_symtab(&self) -> bool {
        self.sym_offset != 0x80000000
    }

    pub fn sym_decl_offset(&self) -> u32 {
        self.sym_decl_offset
    }

    pub fn flag(&self) -> ObjCodeFlag {
        self.special_flag
    }
}

#[derive(Debug, Clone)]
pub struct ObjInitHunk {
    code: Vec<u8>,
}

impl Deref for ObjInitHunk {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct ObjDataHunk {
    name_id: u32,
    sym_offset: u32,
    sym_decl_offset: u32,
    data: Vec<u8>,
}

impl Deref for ObjDataHunk {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl ObjDataHunk {
    pub fn sym_offset(&self) -> u32 {
        self.sym_offset
    }

    pub fn sym_decl_offset(&self) -> u32 {
        self.sym_decl_offset
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct ObjEntryHunk {
    name_id: u32,
    offset: u32,
}

impl ObjEntryHunk {
    pub fn offset(&self) -> u32 {
        self.offset
    }
}

#[derive(Debug, Clone)]
pub struct ObjXRefPair {
    offset: u32,
    value: u32,
}

impl ObjXRefPair {
    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct ObjXRefHunk {
    name_id: u32,
    pairs: Vec<ObjXRefPair>,
}

impl Deref for ObjXRefHunk {
    type Target = Vec<ObjXRefPair>;

    fn deref(&self) -> &Self::Target {
        &self.pairs
    }
}

#[derive(Debug, Clone)]
pub struct ObjExceptInfo {
    info: Vec<u8>,
}

impl Deref for ObjExceptInfo {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct ObjContainerHunk {
    name_id: u32,
    old_def_version: u32,
    old_imp_version: u32,
    current_version: u32,
}

impl ObjContainerHunk {
    pub fn old_def_version(&self) -> u32 {
        self.old_def_version
    }

    pub fn old_imp_version(&self) -> u32 {
        self.old_imp_version
    }

    pub fn current_version(&self) -> u32 {
        self.current_version
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct ObjImportHunk {
    name_id: u32,
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct DataPointerHunk {
    name_id: u32,
    data_name: u32,
}

impl DataPointerHunk {
    pub fn data_name_id(&self) -> u32 {
        self.data_name
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct XPointerHunk {
    name_id: u32,
    xvector_name: u32,
}

impl XPointerHunk {
    pub fn xvector_name(&self) -> u32 {
        self.xvector_name
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct XVectorHunk {
    name_id: u32,
    function_name: u32,
}

impl XVectorHunk {
    pub fn function_name(&self) -> u32 {
        self.function_name
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct ObjSourceHunk {
    name_id: u32,
    moddate: DateTime<Local>,
}
impl ObjSourceHunk {
    pub fn moddate(&self) -> DateTime<Local> {
        self.moddate
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct ObjSegHunk {
    name_id: u32,
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct ObjMethHunk {
    name_id: u32,
    size: u32,
}
impl ObjMethHunk {
    pub fn size(&self) -> u32 {
        self.size
    }
}

#[derive(Debug, Clone)]
pub struct ObjClassPair {
    base_id: u32,
    bias: u32,
}
impl ObjClassPair {
    pub fn base_id(&self) -> u32 {
        self.base_id
    }

    pub fn bias(&self) -> u32 {
        self.bias
    }
}

#[derive(NameIdFromObject, Debug, Clone)]
pub struct ObjClassHunk {
    name_id: u32,
    methods: u16,
    pairs: Vec<ObjClassPair>,
}

impl Deref for ObjClassHunk {
    type Target = Vec<ObjClassPair>;

    fn deref(&self) -> &Self::Target {
        &self.pairs
    }
}

impl ObjClassHunk {
    pub fn methods(&self) -> u16 {
        self.methods
    }
}

#[derive(Debug, Clone)]
pub enum HunkType {
    Undefined,
    Start(ObjSimpleHunk),
    End(ObjSimpleHunk),
    LocalCode(ObjCodeHunk),
    GlobalCode(ObjCodeHunk),
    LocalUninitializedData(ObjDataHunk),
    GlobalUninitializedData(ObjDataHunk),
    LocalInitializedData(ObjDataHunk),
    GlobalInitializedData(ObjDataHunk),
    LocalFarUninitializedData(ObjDataHunk),
    GlobalFarUninitializedData(ObjDataHunk),
    LocalFarInitializedData(ObjDataHunk),
    GlobalFarInitializedData(ObjDataHunk),
    XRefCodeJT16Bit(ObjXRefHunk),
    XRefData16Bit(ObjXRefHunk),
    XRef32Bit(ObjXRefHunk),
    LibraryBreak(ReservedHunk),
    GlobalEntry(ObjEntryHunk),
    LocalEntry(ObjEntryHunk),
    Diff8Bit(ReservedHunk),
    Diff16Bit(ReservedHunk),
    Diff32Bit(ReservedHunk),
    Segment(ObjSegHunk), // m68k-only
    InitCode(ObjInitHunk),
    DeInitCode(ReservedHunk),
    GlobalMultiDef(ObjSimpleHunk),
    GlobalOverload(ObjSimpleHunk),
    XRefCode16Bit(ObjXRefHunk),
    XRefCode32Bit(ObjXRefHunk),
    ForceActive(ReservedHunk), // PPC-only
    GlobalDataPointer(DataPointerHunk),
    GlobalXPointer(XPointerHunk),
    GlobalXVector(XVectorHunk),
    XRefPCRelative32Bit(ObjXRefHunk),
    Illegal1(ReservedHunk),
    Illegal2(ReservedHunk),
    CFMExport(ObjSimpleHunk),
    CFMImport(ObjImportHunk),
    CFMImportContainer(ObjContainerHunk),
    SrcBreak(ObjSourceHunk),
    LocalDataPointer(DataPointerHunk),
    LocalXPointer(XPointerHunk),
    LocalXVector(XVectorHunk),
    ExceptionInfo(ObjExceptInfo),
    CFMInternal(ReservedHunk),
    MethodReference(ObjMethHunk),
    MethodClassDefinition(ObjClassHunk),
    XRefAmbiguous16Bit(ObjXRefHunk),
    WeakImportContainer(ObjContainerHunk),
}

#[derive(Debug, Clone)]
pub struct Hunk {
    hunk: HunkType,
}

#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RawHunkType {
    HUNK_START = 0x4567,
    HUNK_END,
    HUNK_LOCAL_CODE,
    HUNK_GLOBAL_CODE,
    HUNK_LOCAL_UDATA,
    HUNK_GLOBAL_UDATA,
    HUNK_LOCAL_IDATA,
    HUNK_GLOBAL_IDATA,
    HUNK_LOCAL_FARUDATA,
    HUNK_GLOBAL_FARUDATA, // 0x457x
    HUNK_LOCAL_FARIDATA,
    HUNK_GLOBAL_FARIDATA,
    HUNK_XREF_CODEJT16BIT,
    HUNK_XREF_DATA16BIT,
    HUNK_XREF_32BIT,
    HUNK_LIBRARY_BREAK,
    HUNK_GLOBAL_ENTRY,
    HUNK_LOCAL_ENTRY,
    HUNK_DIFF_8BIT,
    HUNK_DIFF_16BIT,
    HUNK_DIFF_32BIT,
    HUNK_SEGMENT,
    HUNK_INIT_CODE,
    HUNK_DEINIT_CODE,
    HUNK_MULTIDEF_GLOBAL,
    HUNK_OVERLOAD_GLOBAL, // 0x458x
    HUNK_XREF_CODE16BIT,
    HUNK_XREF_CODE32BIT,
    HUNK_FORCE_ACTIVE,
    HUNK_GLOBAL_DATAPOINTER,
    HUNK_GLOBAL_XPOINTER,
    HUNK_GLOBAL_XVECTOR,
    HUNK_XREF_PCREL32BIT,
    HUNK_ILLEGAL1,
    HUNK_ILLEGAL2,
    HUNK_CFM_EXPORT,
    HUNK_CFM_IMPORT,
    HUNK_CFM_IMPORT_CONTAINER,
    HUNK_SRC_BREAK,
    HUNK_LOCAL_DATAPOINTER,
    HUNK_LOCAL_XPOINTER,
    HUNK_LOCAL_XVECTOR, // 0x459x
    HUNK_EXCEPTION_INFO,
    HUNK_CFM_INTERNAL,
    HUNK_METHOD_REF,
    HUNK_METHOD_CLASS_DEF,
    HUNK_XREF_AMBIGUOUS16BIT,
    HUNK_WEAK_IMPORT_CONTAINER,
}

#[derive(Debug)]
enum HunkParseState {
    ParseTag,
    ParseObjSimpleHunk(RawHunkType),

    ParseObjCodeHunk(RawHunkType),

    ParseInitCodeHunk(RawHunkType),
    ParseDataHunk(RawHunkType),
    ParseAltEntryHunk(RawHunkType),
    ParseXRefHunk(RawHunkType),
    ParseExceptInfoHunk(RawHunkType),

    ParseObjContainerHunk(RawHunkType),
    ParseObjImportHunk(RawHunkType),
    ParseDataPointerHunk(RawHunkType),
    ParseXPointerHunk(RawHunkType),
    ParseXVectorHunk(RawHunkType),

    ParseObjSourceHunk(RawHunkType),
    ParseObjSegmentHunk(RawHunkType),

    ParseObjMethHunk(RawHunkType),
    ParseObjClassHunk(RawHunkType),

    ParseReservedHunk(RawHunkType),

    CommitHunk(Hunk),

    End,
}

impl PartialEq for HunkParseState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::CommitHunk(_), Self::CommitHunk(_)) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Default for HunkParseState {
    fn default() -> Self {
        HunkParseState::ParseTag
    }
}

impl TryFrom<u16> for HunkParseState {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == RawHunkType::HUNK_START as u16 => {
                Ok(HunkParseState::ParseObjSimpleHunk(RawHunkType::HUNK_START))
            }
            x if x == RawHunkType::HUNK_END as u16 => {
                Ok(HunkParseState::ParseObjSimpleHunk(RawHunkType::HUNK_END))
            }
            x if x == RawHunkType::HUNK_LOCAL_CODE as u16 => Ok(HunkParseState::ParseObjCodeHunk(
                RawHunkType::HUNK_LOCAL_CODE,
            )),
            x if x == RawHunkType::HUNK_GLOBAL_CODE as u16 => Ok(HunkParseState::ParseObjCodeHunk(
                RawHunkType::HUNK_GLOBAL_CODE,
            )),
            x if x == RawHunkType::HUNK_LOCAL_UDATA as u16 => {
                Ok(HunkParseState::ParseDataHunk(RawHunkType::HUNK_LOCAL_UDATA))
            }
            x if x == RawHunkType::HUNK_GLOBAL_UDATA as u16 => Ok(HunkParseState::ParseDataHunk(
                RawHunkType::HUNK_GLOBAL_UDATA,
            )),
            x if x == RawHunkType::HUNK_LOCAL_IDATA as u16 => {
                Ok(HunkParseState::ParseDataHunk(RawHunkType::HUNK_LOCAL_IDATA))
            }
            x if x == RawHunkType::HUNK_GLOBAL_IDATA as u16 => Ok(HunkParseState::ParseDataHunk(
                RawHunkType::HUNK_GLOBAL_IDATA,
            )),
            x if x == RawHunkType::HUNK_LOCAL_FARUDATA as u16 => Ok(HunkParseState::ParseDataHunk(
                RawHunkType::HUNK_LOCAL_FARUDATA,
            )),
            x if x == RawHunkType::HUNK_GLOBAL_FARUDATA as u16 => Ok(
                HunkParseState::ParseDataHunk(RawHunkType::HUNK_GLOBAL_FARUDATA),
            ),
            x if x == RawHunkType::HUNK_LOCAL_FARIDATA as u16 => Ok(HunkParseState::ParseDataHunk(
                RawHunkType::HUNK_LOCAL_FARIDATA,
            )),
            x if x == RawHunkType::HUNK_GLOBAL_FARIDATA as u16 => Ok(
                HunkParseState::ParseDataHunk(RawHunkType::HUNK_GLOBAL_FARIDATA),
            ),

            x if x == RawHunkType::HUNK_XREF_CODEJT16BIT as u16 => Ok(
                HunkParseState::ParseXRefHunk(RawHunkType::HUNK_XREF_CODEJT16BIT),
            ),
            x if x == RawHunkType::HUNK_XREF_DATA16BIT as u16 => Ok(HunkParseState::ParseXRefHunk(
                RawHunkType::HUNK_XREF_DATA16BIT,
            )),
            x if x == RawHunkType::HUNK_XREF_32BIT as u16 => {
                Ok(HunkParseState::ParseXRefHunk(RawHunkType::HUNK_XREF_32BIT))
            }
            x if x == RawHunkType::HUNK_LIBRARY_BREAK as u16 => Ok(
                HunkParseState::ParseReservedHunk(RawHunkType::HUNK_LIBRARY_BREAK),
            ),
            x if x == RawHunkType::HUNK_GLOBAL_ENTRY as u16 => Ok(
                HunkParseState::ParseAltEntryHunk(RawHunkType::HUNK_GLOBAL_ENTRY),
            ),
            x if x == RawHunkType::HUNK_LOCAL_ENTRY as u16 => Ok(
                HunkParseState::ParseAltEntryHunk(RawHunkType::HUNK_LOCAL_ENTRY),
            ),
            x if x == RawHunkType::HUNK_DIFF_8BIT as u16 => Ok(HunkParseState::ParseReservedHunk(
                RawHunkType::HUNK_DIFF_8BIT,
            )),
            x if x == RawHunkType::HUNK_DIFF_16BIT as u16 => Ok(HunkParseState::ParseReservedHunk(
                RawHunkType::HUNK_DIFF_16BIT,
            )),
            x if x == RawHunkType::HUNK_DIFF_32BIT as u16 => Ok(HunkParseState::ParseReservedHunk(
                RawHunkType::HUNK_DIFF_32BIT,
            )),
            x if x == RawHunkType::HUNK_SEGMENT as u16 => Ok(HunkParseState::ParseObjSegmentHunk(
                RawHunkType::HUNK_SEGMENT,
            )),
            x if x == RawHunkType::HUNK_INIT_CODE as u16 => Ok(HunkParseState::ParseInitCodeHunk(
                RawHunkType::HUNK_INIT_CODE,
            )),
            x if x == RawHunkType::HUNK_DEINIT_CODE as u16 => Ok(
                HunkParseState::ParseReservedHunk(RawHunkType::HUNK_DEINIT_CODE),
            ),
            x if x == RawHunkType::HUNK_MULTIDEF_GLOBAL as u16 => Ok(
                HunkParseState::ParseObjSimpleHunk(RawHunkType::HUNK_MULTIDEF_GLOBAL),
            ),
            x if x == RawHunkType::HUNK_OVERLOAD_GLOBAL as u16 => Ok(
                HunkParseState::ParseObjSimpleHunk(RawHunkType::HUNK_OVERLOAD_GLOBAL),
            ),
            x if x == RawHunkType::HUNK_XREF_CODE16BIT as u16 => Ok(HunkParseState::ParseXRefHunk(
                RawHunkType::HUNK_XREF_CODE16BIT,
            )),
            x if x == RawHunkType::HUNK_XREF_CODE32BIT as u16 => Ok(HunkParseState::ParseXRefHunk(
                RawHunkType::HUNK_XREF_CODE32BIT,
            )),
            x if x == RawHunkType::HUNK_FORCE_ACTIVE as u16 => Ok(
                HunkParseState::ParseReservedHunk(RawHunkType::HUNK_FORCE_ACTIVE),
            ),
            x if x == RawHunkType::HUNK_GLOBAL_DATAPOINTER as u16 => Ok(
                HunkParseState::ParseDataPointerHunk(RawHunkType::HUNK_GLOBAL_DATAPOINTER),
            ),
            x if x == RawHunkType::HUNK_GLOBAL_XPOINTER as u16 => Ok(
                HunkParseState::ParseXPointerHunk(RawHunkType::HUNK_GLOBAL_XPOINTER),
            ),
            x if x == RawHunkType::HUNK_GLOBAL_XVECTOR as u16 => Ok(
                HunkParseState::ParseXVectorHunk(RawHunkType::HUNK_GLOBAL_XVECTOR),
            ),
            x if x == RawHunkType::HUNK_XREF_PCREL32BIT as u16 => Ok(
                HunkParseState::ParseXRefHunk(RawHunkType::HUNK_XREF_PCREL32BIT),
            ),
            x if x == RawHunkType::HUNK_ILLEGAL1 as u16 => Ok(HunkParseState::ParseReservedHunk(
                RawHunkType::HUNK_ILLEGAL1,
            )),
            x if x == RawHunkType::HUNK_ILLEGAL2 as u16 => Ok(HunkParseState::ParseReservedHunk(
                RawHunkType::HUNK_ILLEGAL2,
            )),
            x if x == RawHunkType::HUNK_CFM_EXPORT as u16 => Ok(
                HunkParseState::ParseObjSimpleHunk(RawHunkType::HUNK_CFM_EXPORT),
            ),
            x if x == RawHunkType::HUNK_CFM_IMPORT as u16 => Ok(
                HunkParseState::ParseObjImportHunk(RawHunkType::HUNK_CFM_IMPORT),
            ),
            x if x == RawHunkType::HUNK_CFM_IMPORT_CONTAINER as u16 => Ok(
                HunkParseState::ParseObjContainerHunk(RawHunkType::HUNK_CFM_IMPORT_CONTAINER),
            ),
            x if x == RawHunkType::HUNK_SRC_BREAK as u16 => Ok(HunkParseState::ParseObjSourceHunk(
                RawHunkType::HUNK_SRC_BREAK,
            )),
            x if x == RawHunkType::HUNK_LOCAL_DATAPOINTER as u16 => Ok(
                HunkParseState::ParseDataPointerHunk(RawHunkType::HUNK_LOCAL_DATAPOINTER),
            ),
            x if x == RawHunkType::HUNK_LOCAL_XPOINTER as u16 => Ok(
                HunkParseState::ParseXPointerHunk(RawHunkType::HUNK_LOCAL_XPOINTER),
            ),
            x if x == RawHunkType::HUNK_LOCAL_XVECTOR as u16 => Ok(
                HunkParseState::ParseXVectorHunk(RawHunkType::HUNK_LOCAL_XVECTOR),
            ),
            x if x == RawHunkType::HUNK_EXCEPTION_INFO as u16 => Ok(
                HunkParseState::ParseExceptInfoHunk(RawHunkType::HUNK_EXCEPTION_INFO),
            ),
            x if x == RawHunkType::HUNK_CFM_INTERNAL as u16 => Ok(
                HunkParseState::ParseReservedHunk(RawHunkType::HUNK_CFM_INTERNAL),
            ),
            x if x == RawHunkType::HUNK_METHOD_REF as u16 => Ok(HunkParseState::ParseObjMethHunk(
                RawHunkType::HUNK_METHOD_REF,
            )),
            x if x == RawHunkType::HUNK_METHOD_CLASS_DEF as u16 => Ok(
                HunkParseState::ParseObjClassHunk(RawHunkType::HUNK_METHOD_CLASS_DEF),
            ),
            x if x == RawHunkType::HUNK_XREF_AMBIGUOUS16BIT as u16 => Ok(
                HunkParseState::ParseXRefHunk(RawHunkType::HUNK_XREF_AMBIGUOUS16BIT),
            ),
            x if x == RawHunkType::HUNK_WEAK_IMPORT_CONTAINER as u16 => Ok(
                HunkParseState::ParseObjContainerHunk(RawHunkType::HUNK_WEAK_IMPORT_CONTAINER),
            ),
            _ => Err("Bad branch select for hunk"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodeHunks {
    hunks: Vec<Hunk>,
}

impl Deref for CodeHunks {
    type Target = Vec<Hunk>;

    fn deref(&self) -> &Self::Target {
        &self.hunks
    }
}

impl TryFrom<&[u8]> for CodeHunks {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut data: &[u8] = value;

        let mut hunks: Vec<Hunk> = vec![];

        let mut state: HunkParseState = HunkParseState::default();
        while state != HunkParseState::End {
            state = match state {
                HunkParseState::ParseTag => {
                    let tag = convert_be_u16(&data[0..2].try_into().unwrap());

                    data = &data[2..];

                    HunkParseState::try_from(tag).unwrap()
                }
                HunkParseState::ParseObjSimpleHunk(tag) => {
                    let hunk = match tag {
                        RawHunkType::HUNK_START => HunkType::Start(ObjSimpleHunk {}),
                        RawHunkType::HUNK_END => HunkType::End(ObjSimpleHunk {}),

                        RawHunkType::HUNK_MULTIDEF_GLOBAL => {
                            HunkType::GlobalMultiDef(ObjSimpleHunk {})
                        }
                        RawHunkType::HUNK_OVERLOAD_GLOBAL => {
                            HunkType::GlobalOverload(ObjSimpleHunk {})
                        }

                        RawHunkType::HUNK_CFM_EXPORT => HunkType::CFMExport(ObjSimpleHunk {}),

                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseReservedHunk(tag) => {
                    let hunk = match tag {
                        RawHunkType::HUNK_LIBRARY_BREAK => HunkType::LibraryBreak(ReservedHunk {}),

                        RawHunkType::HUNK_DIFF_8BIT => HunkType::Diff8Bit(ReservedHunk {}),
                        RawHunkType::HUNK_DIFF_16BIT => HunkType::Diff16Bit(ReservedHunk {}),
                        RawHunkType::HUNK_DIFF_32BIT => HunkType::Diff32Bit(ReservedHunk {}),

                        RawHunkType::HUNK_DEINIT_CODE => HunkType::DeInitCode(ReservedHunk {}),

                        RawHunkType::HUNK_ILLEGAL1 => HunkType::Illegal1(ReservedHunk {}),
                        RawHunkType::HUNK_ILLEGAL2 => HunkType::Illegal2(ReservedHunk {}),

                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    return Err(format!("Encountered Reserved Hunk: {:?}", hunk));

                    // Commit nothing cause we found reserved hunks we can't process
                    // HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseObjCodeHunk(tag) => {
                    let special = match &hunks.last().unwrap().hunk {
                        HunkType::CFMExport(_) => ObjCodeFlag::CFMExport,
                        HunkType::GlobalOverload(_) => ObjCodeFlag::GlobalOverload,
                        HunkType::GlobalMultiDef(_) => ObjCodeFlag::GlobalMultiDef,
                        _ => ObjCodeFlag::None,
                    };

                    let name_id = convert_be_u32(&data[0..4].try_into().unwrap());
                    let size = convert_be_u32(&data[4..8].try_into().unwrap());
                    let sym_offset = convert_be_u32(&data[8..12].try_into().unwrap());
                    let sym_decl_offset = convert_be_u32(&data[12..16].try_into().unwrap());

                    data = &data[16..];
                    let code = &data[0..size as usize];
                    data = &data[size as usize..];

                    let obj_hunk = ObjCodeHunk {
                        name_id: name_id,
                        sym_offset: sym_offset,
                        sym_decl_offset: sym_decl_offset,
                        code: code.to_owned(),
                        special_flag: special,
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_LOCAL_CODE => HunkType::LocalCode(obj_hunk),
                        RawHunkType::HUNK_GLOBAL_CODE => HunkType::GlobalCode(obj_hunk),

                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseInitCodeHunk(tag) => {
                    let size = convert_be_u32(&data[0..4].try_into().unwrap());

                    data = &data[4..];
                    let code = &data[0..size as usize];
                    data = &data[size as usize..];

                    let obj_hunk = ObjInitHunk {
                        code: code.to_owned(),
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_INIT_CODE => HunkType::InitCode(obj_hunk),

                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }

                HunkParseState::ParseDataHunk(tag) => {
                    let name_id = convert_be_u32(&data[0..4].try_into().unwrap());
                    let size = convert_be_u32(&data[4..8].try_into().unwrap());
                    let sym_offset = convert_be_u32(&data[8..12].try_into().unwrap());
                    let sym_decl_offset = convert_be_u32(&data[12..16].try_into().unwrap());

                    data = &data[16..];

                    // Capture initialized data
                    let code = match tag {
                        RawHunkType::HUNK_GLOBAL_IDATA
                        | RawHunkType::HUNK_LOCAL_IDATA
                        | RawHunkType::HUNK_GLOBAL_FARIDATA
                        | RawHunkType::HUNK_LOCAL_FARIDATA => {
                            let c = &data[0..size as usize];
                            data = &data[size as usize..];
                            c
                        }
                        _ => <&[u8]>::default(),
                    };

                    let obj_hunk = ObjDataHunk {
                        name_id: name_id,
                        sym_offset: sym_offset,
                        sym_decl_offset: sym_decl_offset,
                        data: code.to_owned(),
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_GLOBAL_IDATA => HunkType::GlobalInitializedData(obj_hunk),
                        RawHunkType::HUNK_GLOBAL_UDATA => {
                            HunkType::GlobalUninitializedData(obj_hunk)
                        }
                        RawHunkType::HUNK_LOCAL_IDATA => HunkType::LocalInitializedData(obj_hunk),
                        RawHunkType::HUNK_LOCAL_UDATA => HunkType::LocalUninitializedData(obj_hunk),
                        RawHunkType::HUNK_GLOBAL_FARIDATA => {
                            HunkType::GlobalFarInitializedData(obj_hunk)
                        }
                        RawHunkType::HUNK_GLOBAL_FARUDATA => {
                            HunkType::GlobalFarUninitializedData(obj_hunk)
                        }
                        RawHunkType::HUNK_LOCAL_FARIDATA => {
                            HunkType::LocalFarInitializedData(obj_hunk)
                        }
                        RawHunkType::HUNK_LOCAL_FARUDATA => {
                            HunkType::LocalFarUninitializedData(obj_hunk)
                        }
                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseAltEntryHunk(tag) => {
                    let name_id = convert_be_u32(&data[0..4].try_into().unwrap());
                    let offset = convert_be_u32(&data[4..8].try_into().unwrap());

                    data = &data[8..];

                    let entry_hunk = ObjEntryHunk {
                        name_id: name_id,
                        offset: offset,
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_GLOBAL_ENTRY => HunkType::GlobalEntry(entry_hunk),
                        RawHunkType::HUNK_LOCAL_ENTRY => HunkType::LocalEntry(entry_hunk),
                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseXRefHunk(tag) => {
                    let name_id = convert_be_u32(&data[0..4].try_into().unwrap());
                    let num_pairs = convert_be_u16(&data[4..6].try_into().unwrap());

                    data = &data[6..];

                    // process pairs
                    let mut pairs: Vec<ObjXRefPair> = vec![];
                    for _idx in 0..num_pairs {
                        let offset = convert_be_u32(&data[0..4].try_into().unwrap());
                        let value = convert_be_u32(&data[4..8].try_into().unwrap());

                        pairs.push(ObjXRefPair {
                            offset: offset,
                            value: value,
                        });

                        data = &data[8..]
                    }

                    let xref_hunk = ObjXRefHunk {
                        name_id: name_id,
                        pairs: pairs,
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_XREF_CODEJT16BIT => HunkType::XRefCodeJT16Bit(xref_hunk),
                        RawHunkType::HUNK_XREF_DATA16BIT => HunkType::XRefData16Bit(xref_hunk),
                        RawHunkType::HUNK_XREF_CODE16BIT => HunkType::XRefCode16Bit(xref_hunk),
                        RawHunkType::HUNK_XREF_32BIT => HunkType::XRef32Bit(xref_hunk),
                        RawHunkType::HUNK_XREF_CODE32BIT => HunkType::XRefCode32Bit(xref_hunk),
                        RawHunkType::HUNK_XREF_PCREL32BIT => {
                            HunkType::XRefPCRelative32Bit(xref_hunk)
                        }
                        RawHunkType::HUNK_XREF_AMBIGUOUS16BIT => {
                            HunkType::XRefAmbiguous16Bit(xref_hunk)
                        }

                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseExceptInfoHunk(tag) => {
                    let size = convert_be_u32(&data[0..4].try_into().unwrap());

                    data = &data[4..];
                    let code = &data[0..size as usize];
                    data = &data[size as usize..];

                    let exp_hunk = ObjExceptInfo {
                        info: code.to_vec(),
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_EXCEPTION_INFO => HunkType::ExceptionInfo(exp_hunk),

                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseObjContainerHunk(tag) => {
                    let name_id = convert_be_u32(&data[0..4].try_into().unwrap());
                    let old_def_version = convert_be_u32(&data[4..8].try_into().unwrap());
                    let old_imp_version = convert_be_u32(&data[8..12].try_into().unwrap());
                    let current_version = convert_be_u32(&data[12..16].try_into().unwrap());

                    data = &data[16..];

                    let objc_hunk = ObjContainerHunk {
                        name_id: name_id,
                        old_def_version: old_def_version,
                        old_imp_version: old_imp_version,
                        current_version: current_version,
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_CFM_IMPORT_CONTAINER => {
                            HunkType::CFMImportContainer(objc_hunk)
                        }
                        RawHunkType::HUNK_WEAK_IMPORT_CONTAINER => {
                            HunkType::WeakImportContainer(objc_hunk)
                        }

                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseObjImportHunk(tag) => {
                    let name_id = convert_be_u32(&data[0..4].try_into().unwrap());

                    data = &data[4..];

                    let obj_hunk = ObjImportHunk { name_id: name_id };

                    let hunk = match tag {
                        RawHunkType::HUNK_CFM_IMPORT => HunkType::CFMImport(obj_hunk),

                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseDataPointerHunk(tag) => {
                    let name_id: u32 = convert_be_u32(&data[0..4].try_into().unwrap());
                    let d_name: u32 = convert_be_u32(&data[4..8].try_into().unwrap());

                    data = &data[8..];

                    let dp_hunk = DataPointerHunk {
                        name_id: name_id,
                        data_name: d_name,
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_LOCAL_DATAPOINTER => HunkType::LocalDataPointer(dp_hunk),
                        RawHunkType::HUNK_GLOBAL_DATAPOINTER => {
                            HunkType::GlobalDataPointer(dp_hunk)
                        }
                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseXPointerHunk(tag) => {
                    let xp_name: u32 = convert_be_u32(&data[0..4].try_into().unwrap());
                    let xv_name: u32 = convert_be_u32(&data[4..8].try_into().unwrap());

                    data = &data[8..];

                    let xp_hunk = XPointerHunk {
                        name_id: xp_name,
                        xvector_name: xv_name,
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_LOCAL_XPOINTER => HunkType::LocalXPointer(xp_hunk),
                        RawHunkType::HUNK_GLOBAL_XPOINTER => HunkType::GlobalXPointer(xp_hunk),
                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseXVectorHunk(tag) => {
                    let xv_name: u32 = convert_be_u32(&data[0..4].try_into().unwrap());
                    let f_name: u32 = convert_be_u32(&data[4..8].try_into().unwrap());

                    data = &data[8..];

                    let xv_hunk = XVectorHunk {
                        name_id: xv_name,
                        function_name: f_name,
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_LOCAL_XVECTOR => HunkType::LocalXVector(xv_hunk),
                        RawHunkType::HUNK_GLOBAL_XVECTOR => HunkType::GlobalXVector(xv_hunk),
                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseObjSourceHunk(tag) => {
                    let name_id: u32 = convert_be_u32(&data[0..4].try_into().unwrap());
                    let moddate: u32 = convert_be_u32(&data[4..8].try_into().unwrap());

                    data = &data[8..];

                    let src_hunk = ObjSourceHunk {
                        name_id: name_id,
                        moddate: from_mac_datetime(moddate).into(),
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_SRC_BREAK => HunkType::SrcBreak(src_hunk),
                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseObjSegmentHunk(tag) => {
                    let name_id: u32 = convert_be_u32(&data[0..4].try_into().unwrap());

                    data = &data[4..];

                    let seg_hunk = ObjSegHunk { name_id: name_id };

                    let hunk = match tag {
                        RawHunkType::HUNK_SEGMENT => HunkType::Segment(seg_hunk),
                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseObjMethHunk(tag) => {
                    let name_id: u32 = convert_be_u32(&data[0..4].try_into().unwrap());
                    let size: u32 = convert_be_u32(&data[4..8].try_into().unwrap());

                    data = &data[8..];

                    let meth_hunk = ObjMethHunk {
                        name_id: name_id,
                        size: size,
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_METHOD_REF => HunkType::MethodReference(meth_hunk),
                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }
                HunkParseState::ParseObjClassHunk(tag) => {
                    let name_id = convert_be_u32(&data[0..4].try_into().unwrap());
                    let num_methods = convert_be_u16(&data[4..6].try_into().unwrap());
                    let num_pairs = convert_be_u16(&data[6..8].try_into().unwrap());

                    data = &data[8..];

                    // process pairs
                    let mut pairs: Vec<ObjClassPair> = vec![];
                    for _idx in 0..num_pairs {
                        let base_id = convert_be_u32(&data[0..4].try_into().unwrap());
                        let bias = convert_be_u32(&data[4..8].try_into().unwrap());

                        pairs.push(ObjClassPair {
                            base_id: base_id,
                            bias: bias,
                        });

                        data = &data[8..]
                    }

                    let class_hunk = ObjClassHunk {
                        name_id: name_id,
                        methods: num_methods,
                        pairs: pairs,
                    };

                    let hunk = match tag {
                        RawHunkType::HUNK_METHOD_CLASS_DEF => {
                            HunkType::MethodClassDefinition(class_hunk)
                        }

                        _ => {
                            return Err(format!(
                                "Bad branch selection in {:#?} for tag: {:#?}",
                                state, tag
                            ))
                        }
                    };

                    HunkParseState::CommitHunk(Hunk { hunk: hunk })
                }

                HunkParseState::CommitHunk(hunk) => {
                    hunks.push(hunk);

                    if data.len() == 0 {
                        HunkParseState::End
                    } else {
                        HunkParseState::ParseTag
                    }
                }
                _ => return Err(format!("Bad branch encountered: {:#?}", state)),
            }
        }

        Ok(CodeHunks { hunks: hunks })
    }
}
