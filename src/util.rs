use crate::objects_m68k::MetrowerksObject;
use chrono::{DateTime, Local, NaiveDate, TimeZone, Utc};
use std::{collections::VecDeque, io, io::Write, sync::Once};

pub trait Serializable: for<'a> TryFrom<&'a [u8]> {
    fn serialize_out<W: Write>(&self, writer: &mut W) -> io::Result<()>;

    fn serialize_in(value: &[u8]) -> Result<Self, <Self as TryFrom<&[u8]>>::Error> {
        Self::try_from(value)
    }
}

pub trait NameIdFromObject<'a>: Sized {
    fn name(&'a self, obj: &'a MetrowerksObject) -> &str;
}

pub(crate) trait RawLength: Sized {
    fn raw_length(&self) -> usize;
}

const NAMEHASH: u16 = 1024;

pub fn nametable_hash(name: &str) -> u16 {
    let mut hashval: u16;
    let mut u: u8;
    let s: VecDeque<u8> = name.as_bytes().to_owned().into();

    hashval = (name.len() as u32 & 0x00ff) as u16;

    if hashval != 0 {
        u = 0;
        for c in s.iter() {
            u = (u >> 3) | (u << 5);
            u += *c;
        }
        hashval = (hashval << 8) | (u as u16);
    }

    hashval & (NAMEHASH - 1)
}

pub fn convert_be_u16(data: &[u8; 2]) -> u16 {
    let res: u16 = unsafe { std::mem::transmute(*data) };
    u16::from_be(res)
}

pub fn convert_be_u32(data: &[u8; 4]) -> u32 {
    let res: u32 = unsafe { std::mem::transmute(*data) };
    u32::from_be(res)
}

pub fn convert_be_i16(data: &[u8; 2]) -> i16 {
    let res: i16 = unsafe { std::mem::transmute(*data) };
    i16::from_be(res)
}

pub fn convert_be_i32(data: &[u8; 4]) -> i32 {
    let res: i32 = unsafe { std::mem::transmute(*data) };
    i32::from_be(res)
}

/* Timestamp conversion */
static mut MAC_EPOCH_OFFSET: i64 = 0;
static INIT_MAC_EPOCH_OFFSET: Once = Once::new();

fn get_offset() -> i64 {
    unsafe {
        INIT_MAC_EPOCH_OFFSET.call_once(|| {
            MAC_EPOCH_OFFSET = NaiveDate::from_ymd_opt(1904, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Local)
                .unwrap()
                .timestamp()
                .abs()
        });
        MAC_EPOCH_OFFSET
    }
}

pub fn from_mac_datetime(date: u32) -> DateTime<Utc> {
    // Classic MacOS timestamps start from midnight on January 1, 1904.
    Utc.timestamp_opt((date as i64) - get_offset(), 0).unwrap()
}

pub fn to_mac_datetime<T: TimeZone>(date: DateTime<T>) -> u32 {
    // Classic MacOS timestamps start from midnight on January 1, 1904.
    (date.to_utc().timestamp() + get_offset()) as u32
}
