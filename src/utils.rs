use std::fmt::{Binary, Display, Octal, UpperHex};


pub trait Number: Display + Binary + Octal + UpperHex {
    fn print_bin(&self, word_length: Length) {
        let padding = word_length as usize;
        println!("Binary: {:0>width$b}", self, width = padding);
    }

    fn print_oct(&self, word_length: Length) {
        let padding = (word_length as usize).div_ceil(3);
        println!("Octal: {:0>width$o}", self, width = padding);
    }

    fn print_dec(&self) {
        println!("Decimal: {}", self);
    }

    fn print_hex(&self, word_length: Length) {
        let padding = (word_length as usize)/4;
        println!("Hexadecimal: {:0>width$X}", self, width = padding);
    }

    fn print_all(&self, word_length: &Length) {
        self.print_bin(word_length.clone());
        self.print_oct(word_length.clone());
        self.print_dec();
        self.print_hex(word_length.clone());
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

#[derive(Debug, Clone)]
pub enum Length {
    Byte = 8,
    Word = 16,
    Dword = 32,
    Qword = 64,
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

