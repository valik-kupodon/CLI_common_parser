use clap::{Arg, ArgMatches, Command};

pub fn register_args() -> ArgMatches {
    Command::new("CLI common string worker")
        .version("1.0")
        .author("valentinefilatov2015@gmail.com")
        .about("Work with strings")
        .subcommand(
            Command::new("run")
                .about("Get type of conversion and string conversion")
                .arg(
                    Arg::new("command_type")
                        .help("Type of string conversion: quote_words")
                        .value_parser(clap::builder::PossibleValuesParser::new(["quote_words"]))
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("string")
                        .help("String, that need to convert")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            Command::new("generate_file")
                .about("Generate file from sample text in provided directory")
                .arg(
                    Arg::new("file_path")
                        .help("Path to file")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("repeat_times")
                        .help("How many times you need to repeat provided text")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::new("text")
                        .help("String that will be written into the file")
                        .required(true)
                        .index(3),
                ),
        )
        .get_matches()
}

pub fn get_command_type_and_string_args(sub_m: &ArgMatches) -> (String, String) {
    let packages: Vec<_> = sub_m
        .get_many::<String>("command_type")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
    let command_type = packages.join(", ");
    let packages: Vec<_> = sub_m
        .get_many::<String>("string")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
    let string = packages.join(", ");
    (command_type, string)
}

pub fn get_file_path_text_and_numer_of_repetition(sub_m: &ArgMatches) -> (String, String, String) {
    let packages: Vec<_> = sub_m
        .get_many::<String>("file_path")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
    let file_path = packages.join(", ");
    let packages: Vec<_> = sub_m
        .get_many::<String>("repeat_times")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
    let repeat_times = packages.join(", ");
    let packages: Vec<_> = sub_m
        .get_many::<String>("text")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
    let text = packages.join(", ");

    (file_path, repeat_times, text)
}
