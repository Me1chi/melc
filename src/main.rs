use melc::configio::ConfigFile;
use melc::input::Args;
use clap::Parser;

fn main() {

    let config = ConfigFile::load_config_to_string();
    let config = if let Ok(string) = config {
        toml::from_str(&string).ok()
    } else {
        Some(ConfigFile::new_default())
    };

    let mut safe_config = if let Some(config) = config {
        config.to_safe_config()
    } else {
        ConfigFile::new_default().to_safe_config()
    };

    let args = Args::parse();
    args.adapt_safe_config(&mut safe_config);

    //Time to get the number
    let input_number = args.get_number(
        &safe_config.base,
        safe_config.signed,
        &safe_config.length
    ).expect("Number is in the wrong form"); 

    input_number.print_all();

    if args.is_setter_on() {
        ConfigFile::from_safe_config(safe_config).store_config().expect("Fatal error!");
    }

}
