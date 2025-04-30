use std::{error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

// struct Config {
//     query: String,
//     filename: String,
// }

// fn parse_config(args: &Vec<String>) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config { query, filename }
// }

// optimized version
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: <program> <query> <filename>");
        }

        let query = &args[1];
        let filename = &args[2];
        Ok(Config { query, filename })
    }
}
