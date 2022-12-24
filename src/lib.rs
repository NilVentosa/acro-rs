use clap::{Arg, Command};
use std::fs::File;
use std::{
    error::Error,
    io::{self, BufRead, BufReader},
};
use std::env;

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
                .help("the csv file with the acronyms and definitions"),
        )
        .arg(
            Arg::new("acro_column")
                .short('a')
                .long("acro")
                .help("the column with the acronyms")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("definition_column")
                .short('d')
                .long("definition")
                .help("the column with the definitions")
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    let mut file: String = "".to_string();
    if let Some(f) = matches.get_one::<String>("file") {
        file = f.to_string(); 
    } else if let Ok(f) = env::var("ACRO_FILE") {
        file = f;
    } else {
        eprintln!("File should be specified in argument -f or in env variable ACRO_FILE");
    }

    let mut acro_column: usize = 0;
    if let Some(a) = matches.get_one::<usize>("acro_column") {
        acro_column = a.to_owned() - 1; 
    } else if let Ok(a) = env::var("ACRO_COLUMN") {
        if let Ok(a) =  a.parse::<usize>() {
            acro_column = a - 1;
        }
    } 

    let mut definition_column: usize = 1;
    if let Some(d) = matches.get_one::<usize>("definition_column") {
        definition_column = d.to_owned() - 1; 
    } else if let Ok(d) = env::var("DEFINITION_COLUMN") {
        if let Ok(d) =  d.parse::<usize>() {
            definition_column = d - 1;
        }
    } 

    Ok(Config {
        acronym: matches.get_one::<String>("acronym").unwrap().to_string(),
        file,
        acro_column,
        definition_column,
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
