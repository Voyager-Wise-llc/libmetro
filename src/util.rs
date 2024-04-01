use std::collections::VecDeque;

use crate::objects_m68k::MetrowerksObject;


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
