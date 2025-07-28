use serde::{Deserialize, Serialize};

use crate::utils::{Length, Base, SafeConfig};

#[derive(Deserialize, Serialize)]
pub struct ConfigFile {
    signed: Option<bool>,
    length: Option<u8>,
    base: Option<String>,
}

impl ConfigFile {

    pub fn new_default() -> Self {
        ConfigFile {
            signed: Some(true),
            length: Some(32),
            base: Some(String::from("hex")),
        }
    }

    pub fn load_config_to_string() -> Result<String, Box<dyn std::error::Error>> {
        use std::fs;

        let config_path = dirs_next::config_dir().expect("Configs directory couldn't be found");
        let config_path = config_path.join("melc");

        if config_path.exists() {
            let config_path = config_path.join("config.toml");
            Ok(fs::read_to_string(config_path)?)
        } else {
            Err("Missing melc config folder".into())
        }
    }

    pub fn store_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;
        use std::io::Write;

        let config_path = dirs_next::config_dir().ok_or("Configs directory couldn't be found")?;
        // If this step fails, a critical error is thrown.

        let config_path = config_path.join("melc");

        fs::create_dir_all(&config_path)?;

        let file_path = config_path.join("config.toml");

        let contents = toml::to_string(self)?;

        let mut config_file = fs::File::create(file_path)?;

        config_file.write_all(contents.as_bytes())?;

        println!("Configs updated!!");

        Ok(())
    }

    pub fn to_safe_config(&self) -> SafeConfig {
        let default_config = ConfigFile::new_default();

        SafeConfig {
            signed: if let Some(sign) = self.signed {
                sign
            } else {
                default_config.signed.unwrap()
            },

            length: if let Some(len) = self.length {
                match len {
                    8 => Length::Byte,
                    16 => Length::Word,
                    32 => Length::Dword,
                    64 => Length::Qword,
                    _ => Length::Dword,
                } 
            } else {
                match default_config.length.unwrap() {
                    8 => Length::Byte,
                    16 => Length::Word,
                    32 => Length::Dword,
                    64 => Length::Qword,
                    _ => Length::Dword,
                }
            },
            base: if let Some(b) = &self.base { 
                let b = b.as_str();
                match b {
                    "bin" => Base::Binary,
                    "oct" => Base::Octal,
                    "dec" => Base::Decimal,
                    "hex" => Base::Hexadecimal,
                    _ => Base::Hexadecimal,
                }
            } else {
                match default_config.base.unwrap().as_str() {
                    "bin" => Base::Binary,     
                    "oct" => Base::Octal,      
                    "dec" => Base::Decimal,    
                    "hex" => Base::Hexadecimal,
                    _ => Base::Hexadecimal,
                }
            },
        }
    }

    pub fn from_safe_config(safe_config: SafeConfig) -> Self {

        let length = match safe_config.length {
            Length::Byte => 8,
            Length::Word => 16,
            Length::Dword => 32,
            Length::Qword => 64,
        };

        let base = match safe_config.base {
            Base::Binary => String::from("bin"),
            Base::Octal => String::from("oct"),
            Base::Decimal => String::from("dec"),
            Base::Hexadecimal => String::from("hex"),
        };

        Self {
            signed: Some(safe_config.signed),
            length: Some(length),
            base: Some(base),
        }
    }

}

