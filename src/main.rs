use config::Config;

fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("settings"))
        .build()
        .unwrap();

    println!("{:?}", settings.get_int("acronym_column").unwrap());
}
