use clap::{Arg, Command};
use std::fs::File;
use std::{
    error::Error,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
pub struct Config {
    acronym: String,
    file: String,
    acro_column: usize,
    definition_column: usize,
}

#[derive(Debug)]
pub struct Entry {
    acronym: String,
    definition: String,
}

impl Entry {
    fn print(&self) {
        println!("- {}: {}", self.acronym, self.definition);
    }
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
                .value_parser(clap::value_parser!(usize))
                .default_value("1"),
        )
        .arg(
            Arg::new("definition_column")
                .short('d')
                .long("definition")
                .help("the column with the definitions")
                .value_parser(clap::value_parser!(usize))
                .default_value("2"),
        )
        .get_matches();

    Ok(Config {
        acronym: matches.get_one::<String>("acronym").unwrap().to_string(),
        file: matches.get_one::<String>("file").unwrap().to_string(),
        acro_column: *matches.get_one::<usize>("acro_column").unwrap(),
        definition_column: *matches.get_one::<usize>("definition_column").unwrap(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let entries = get_entries_from_file(&config);
    let other_entries = find_matching_entries(entries, config.acronym);

    for entry in other_entries {
        entry.print();
    }

    Ok(())
}

fn find_matching_entries(entries: Vec<Entry>, acro: String) -> Vec<Entry> {
    entries
        .into_iter()
        .filter(|e| e.acronym.to_uppercase() == acro.to_uppercase())
        .collect()
}

fn get_entries_from_file(config: &Config) -> Vec<Entry> {
    let mut entries = Vec::new();

    match open(&config.file) {
        Err(err) => eprintln!("Failed to open {}: {}", config.file, err),
        Ok(file) => {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file);

            for record in reader.records() {
                let record = record.unwrap();
                let acronym = record.get(config.acro_column - 1);
                let definition = record.get(config.definition_column - 1);

                if acronym.is_some() && definition.is_some() {
                    entries.push(Entry {
                        acronym: acronym.unwrap().to_string(),
                        definition: definition.unwrap().to_string(),
                    });
                }
            }
        }
    }
    entries
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
