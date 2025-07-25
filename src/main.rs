


use clap::{Arg, ArgGroup, Parser, Subcommand};

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
struct Args {

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
    number: Option<String>,
    
}

fn main() {

    let args = Args::parse();

    println!("Argumentos lidos com sucesso!");
}
