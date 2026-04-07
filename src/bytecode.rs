use std::mem;

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Op {
    Push = 0x00,
    Pop = 0x01,

    Load = 0x02,

    Call = 0x03,
    Return = 0x04,

    Write = 0x05,

    Add = 0x06,
    Sub = 0x07,
    Mul = 0x08,
    Div = 0x09,
    Pow = 0x0A,

    Cat = 0x0B,

    Str2Int = 0x0C,
    Int2Str = 0x0D,
    Bool2Str = 0x0E,
    Int2Bool = 0x0F,
    Str2Bool = 0x10
}

impl From<u8> for Op {
    fn from(ip: u8) -> Op {
        if ip <= 0x10 {
            unsafe { mem::transmute(ip) }
        } else {
            panic!("Conversion failed: {}", ip)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(usize)]
pub enum Frame {
    Block = 0x00,
    Func = 0x01
}

impl From<usize> for Frame {
    fn from(ip: usize) -> Frame {
        if ip <= 0x01 {
            unsafe { mem::transmute(ip) }
        } else {
            panic!("Conversion failed: {}", ip)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum NumSize {
    _8 = 0x00,
    _32 = 0x01,
    _64 = 0x02
}

impl From<u8> for NumSize {
    fn from(ip: u8) -> NumSize {
        if ip <= 0x02 {
            unsafe { mem::transmute(ip) }
        } else {
            panic!("Conversion failed: {}", ip)
        }
    }
}
