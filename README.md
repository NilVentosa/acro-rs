# acro-rs
Helps query csv files of acronyms

Usage: acro [OPTIONS] --file <file> [acronym]

Arguments:  
- [acronym]  the acronym to query

Options:  
  - -f, --file <file>                     the csv file with the acronyms and definitions  
  - -a, --acro <acro_column>              the column with the acronyms [default: 1]  
  - -d, --definition <definition_column>  the column with the definitions [default: 2]  
  - -h, --help                            Print help information  
  - -V, --version                         Print version information  

## TODO
- [ ] Setting in env variables
- [ ] Header selector
- [ ] Separator selector
- [ ] Unit tests
