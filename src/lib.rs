use clap::{Arg, Command};
use std::fs::File;
use std::{
    error::Error,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Config {
    acronym: String,
    file: String,
    acro_column: u8,
    definition_column: u8,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("Acro")
        .author("Nil Ventosa")
        .version("0.1.0")
        .about("Helps query csv files of acronyms")
        .arg(
            Arg::new("acronym")
                .help("the acronym to query")
                .required(true),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .required(true)
                .help("the csv file with the acronyms and definitions"),
        )
        .arg(
            Arg::new("acro_column")
                .short('a')
                .long("acro")
                .help("the column with the acronyms")
                .value_parser(clap::value_parser!(u8))
                .default_value("1"),
        )
        .arg(
            Arg::new("definition_column")
                .short('d')
                .long("definition")
                .help("the column with the definitions")
                .value_parser(clap::value_parser!(u8))
                .default_value("2"),
        )
        .get_matches();

    Ok(Config {
        acronym: matches.get_one::<String>("acronym").unwrap().to_string(),
        file: matches.get_one::<String>("file").unwrap().to_string(),
        acro_column: matches.get_one::<u8>("acro_column").unwrap(),
        definition_column: *matches.get_one::<u8>("definition_column").unwrap(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    match open(&config.file) {
        Err(err) => eprintln!("Failed to open {}: {}", config.file, err),
        Ok(file) => {
            for line in file.lines() {
                if let Ok(entry) = line {
                    if entry.contains(&config.acronym) {
                        println!("{}", entry);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    Ok(Box::new(BufReader::new(File::open(filename)?)))
}
