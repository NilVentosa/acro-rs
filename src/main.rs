fn main() {
    if let Err(e) = acro::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
