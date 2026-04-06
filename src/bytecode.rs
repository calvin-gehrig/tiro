use std::mem;

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Op {
    Push = 0x01,
    Pop = 0x02,

    Load = 0x03,

    Call = 0x04,
    Return = 0x05,

    Write = 0x06,

    Add = 0x07,
    Sub = 0x08,
    Mul = 0x09,
    Div = 0x0A,
    Pow = 0x0B,

    Cat = 0x0C,
}

impl From<u8> for Op {
    fn from(ip: u8) -> Op {
        if ip <= 0x0C {
            unsafe { mem::transmute(ip) }
        } else {
            panic!("Conversion failed")
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
            panic!("Conversion failed")
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
            panic!("Conversion failed")
        }
    }
}
