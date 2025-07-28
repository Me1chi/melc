use clap::{ArgGroup, Parser};

use crate::utils::{Base, Length, SafeConfig, Number};

#[derive(Parser, Debug)]
#[command(
    name = "melc",
    version = "0.1",
    author = "Melchior Boaretto Neto",
    about = "Simple usual bases converter",
    group(ArgGroup::new("base")),
    group(ArgGroup::new("sign")),
    group(ArgGroup::new("size")),
)]
pub struct Args {

    // Base arguments
    #[arg(short = 'b', long = "bin", group = "base", help = "Binary base")]
    bin: bool,

    #[arg(short = 'o', long = "oct", group = "base", help = "Octal base")]
    oct: bool,

    #[arg(short = 'd', long = "dec", group = "base", help = "Decimal base")]
    dec: bool,

    #[arg(short = 'x', long = "hex", group = "base", help = "Hexadecimal base")]
    hex: bool,


    // Sign arguments
    #[arg(short = 'u', group = "sign", help = "unsigned")]
    unsigned: bool,
    
    #[arg(short = 's', group = "sign", help = "signed")]
    signed: bool,


    // Length arguments
    #[arg(long = "byte", group = "size")]
    byte: bool,
    
    #[arg(long = "word", group = "size")]
    word: bool,
    
    #[arg(long = "dword", group = "size")]
    dword: bool,

    #[arg(long = "qword", group = "size")]
    qword: bool,

    // Set argument
    #[arg(long = "set", help = "change the config file")]
    setter: bool,

    // Number argument (Will be a string)
    #[arg(allow_hyphen_values = true)]
    number: String,
}

impl Args {

    pub fn is_setter_on(&self) -> bool {
        self.setter
    }

    pub fn get_base(&self) -> Option<Base>{
        if self.bin {
            Some(Base::Binary)
        } else if self.oct {
            Some(Base::Octal)
        } else if self.dec {
            Some(Base::Decimal)
        } else if self.hex {
            Some(Base::Hexadecimal)
        } else {
            None
        }
    }

    pub fn is_signed(&self) -> Option<bool> {
        if self.unsigned {
            Some(false)
        } else if self.signed {
            Some(true)
        } else {
            None
        }
    }

    pub fn get_length(&self) -> Option<Length> {
        if self.byte {
            Some(Length::Byte)
        } else if self.word {
            Some(Length::Word)
        } else if self.dword {
            Some(Length::Dword)
        } else if self.qword {
            Some(Length::Qword) 
        } else {
            None
        }
    }

    pub fn adapt_safe_config(&self, safe_config: &mut SafeConfig) {
        if let Some(sign) = self.is_signed() {
            safe_config.signed = sign;
        }

        if let Some(base) = self.get_base() {
            safe_config.base = base;
        }

        if let Some(length) = self.get_length() {
            safe_config.length = length;
        }
    }

    pub fn get_number(&self, base: &Base, signed: bool, word_length: &Length) -> Option<Box<dyn Number>> {
        let base = base.clone() as u32;
        let word= word_length;

        match word {
            Length::Byte => {
                let number = u8::from_str_radix(&self.number, base).ok()?;
                if !signed {
                    Some(Box::new(number))
                } else {
                    Some(Box::new(number as i8))
                }
            },
            Length::Word => {
                let number = u16::from_str_radix(&self.number, base).ok()?;
                if !signed {
                    Some(Box::new(number))
                } else {
                    Some(Box::new(number as i16))
                }
            },
            Length::Dword => {
                let number = u32::from_str_radix(&self.number, base).ok()?;
                if !signed {
                    Some(Box::new(number))
                } else {
                    Some(Box::new(number as i32))
                }
            },
            Length::Qword => {
                let number = u64::from_str_radix(&self.number, base).ok()?;
                if !signed {
                    Some(Box::new(number))
                } else {
                    Some(Box::new(number as i64))
                }
            },
        }
    }
}
