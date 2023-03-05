#![allow(dead_code)]
// A Simple Config Struct

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum ConfigOrSomething {
    Config(Config),
}

#[derive(Debug, serde::Deserialize)]
struct Config {
    value: Option<i64>,
    strings: Option<Vec<String>>,
}

fn main() -> std::io::Result<()> {
    let config_str = "value: 42\nstrings:\n - A\n - B\n - null\n";

    let c_enum = serde_yaml::from_str::<ConfigOrSomething>(config_str);
    eprintln!("c_enum : {:#?}", c_enum);

    let c_struct = serde_yaml::from_str::<Config>(config_str);
    eprintln!("c_struct : {:#?}", c_struct);

    Ok(())
}
