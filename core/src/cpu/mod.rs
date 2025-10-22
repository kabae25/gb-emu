use crate::utils::*;

pub struct Cpu {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

#[derive(Copy, Clone)]
pub enum Regs {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Copy, Clone)]
pub enum Regs16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

pub enum Flags {
    Z,
    N,
    H,
    C,
}

impl Cpu {
    pub fn new() -> Self {
        pc: 0x0000,
        sp: 0x0000,
        a: 0x00,
        b: 0x00,
        c: 0x00,
        d: 0x00,
        e: 0x00,
        f: 0x00,
        h: 0x00,
        l: 0x00,
    }

    pub fn get_r8(&self, r: Regs) -> u8 {
        match r {
            Regs8::A => { self.a },
            Regs8::B => { self.b },
            Regs8::C => { self.c },
            Regs8::D => { self.d },
            Regs8::E => { self.e },
            Regs8::F => { self.f },
            Regs8::H => { self.h },
            Regs8::L => { self.l },
        }
    }

    pub fn set_r8(&mut self, r: Regs, val: u8) {
        match r {
            Regs8::A => {self.a = val},
            Regs8::B => {self.b = val},
            Regs8::C => {self.c = val},
            Regs8::D => {self.d = val},
            Regs8::E => {self.e = val},
            Regs8::F => {self.f = val & 0xF0}, // The bottom four bits of F will always be 0
            Regs8::H => {self.h = val},
            Regs8::L => {self.l = val},
        }
    }

    pub fn get_r16(&self, r: Regs16) -> u16 {
        match r {
            Regs16::AF => { merge_bytes(self.a, self.f) },
            Regs16::BC
            Regs16::DE
            Regs16::HL
            Regs16::SP
        }
    }

    pub fn set_r16(&mut self, r: Regs16, val: u16) {

    }
}

