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
        acro_column: *matches.get_one::<usize>("acro_column").unwrap() - 1,
        definition_column: *matches.get_one::<usize>("definition_column").unwrap() - 1,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let all_entries = get_entries_from_file(&config);
    let matching_entries = find_matching_entries(&all_entries, config.acronym);

    for entry in matching_entries {
        entry.print();
    }

    Ok(())
}

fn find_matching_entries(entries: &Vec<Entry>, acro: String) -> Vec<Entry> {
    let matching = find_exact_match(entries, acro.clone());

    if matching.is_empty() {
        return find_partial_match(entries, acro);
    }
    matching
}

fn find_exact_match(entries: &Vec<Entry>, acro: String) -> Vec<Entry> {
    let mut matching = Vec::new();

    for entry in entries {
        if entry.acronym.to_uppercase() == acro.to_uppercase() {
            matching.push(Entry {
                acronym: entry.acronym.clone(),
                definition: entry.definition.clone(),
            });
        }
    }

    matching
}

fn find_partial_match(entries: &Vec<Entry>, acro: String) -> Vec<Entry> {
    let mut matching = Vec::new();

    for entry in entries {
        if entry.acronym.to_uppercase().contains(&acro.to_uppercase()) {
            matching.push(Entry {
                acronym: entry.acronym.clone(),
                definition: entry.definition.clone(),
            });
        }
    }
    matching
}

fn get_entries_from_file(config: &Config) -> Vec<Entry> {
    let mut entries = Vec::new();

    match open(&config.file) {
        Err(err) => eprintln!("Failed to open {}: {}", config.file, err),
        Ok(file) => {
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file);

            for record in reader.records().flatten() {
                if let (Some(acronym), Some(definition)) = (
                    record.get(config.acro_column),
                    record.get(config.definition_column),
                ) {
                    entries.push(Entry {
                        acronym: acronym.to_string(),
                        definition: definition.to_string(),
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
