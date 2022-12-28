# acro-rs
Helps query csv files of acronyms

Usage: acro [OPTIONS] --file <file> [acronym]

Arguments:  
- [acronym]  the acronym to query

Options:  
  - -f, --file <file>                     the csv file with the acronyms and definitions  
                                          or env variable ACRO_FILE
  - -a, --acro <acro_column>              the column with the acronyms [default: 1]  
                                          or env variable ACRO_COLUMN
  - -d, --definition <definition_column>  the column with the definitions [default: 2]  
                                          or env variable DEFINITION_COLUMN  
  - -c, --color                           when present the output has colors  
                                          or if env variable ACRO_COLOR exists  
  - -H, --header                          when present the first line is treated like a header  
                                          or if env variable ACRO_HEADER exists  
  - -h, --help                            Print help information  
  - -V, --version                         Print version information  

## TODO
- [x] Setting in env variables
- [x] Header selector
- [ ] Separator selector
- [x] Unit tests
- [ ] Column for longer definition
- [x] Colors
