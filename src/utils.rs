use std::fmt::{Binary, Display, Octal, UpperHex};


pub trait Number: Display + Binary + Octal + UpperHex {
    fn print_bin(&self) {
        println!("Binary: {:b}", self);
    }

    fn print_oct(&self) {
        println!("Octal: {:o}", self);
    }

    fn print_dec(&self) {
        println!("Decimal: {}", self);
    }

    fn print_hex(&self) {
        println!("Hexadecimal: {:X}", self);
    }

    fn print_all(&self) {
        self.print_bin();
        self.print_oct();
        self.print_dec();
        self.print_hex();
    }
}

impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}

#[derive(Debug)]
pub enum Length {
    Byte,
    Word,
    Dword,
    Qword,
}

#[repr(u32)]
#[derive(Debug, Clone)]
pub enum Base {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hexadecimal = 16,
}

#[derive(Debug)]
pub struct SafeConfig {
    pub signed: bool,
    pub length: Length,
    pub base: Base,
}

