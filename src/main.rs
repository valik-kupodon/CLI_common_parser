use crate::command_line_interface::{get_command_type_and_string_args, get_file_path_text_and_numer_of_repetition};
use crate::string_converter::QuoteString;
use std::time::Instant;
use crate::file_generation::FileGenerator;

mod command_line_interface;
mod string_converter;
mod file_generation;

fn main() {
    let start = Instant::now();
    let matches = command_line_interface::register_args();
    match matches.subcommand() {
        Some(("run", sub_m)) => {
            let (_, string) = get_command_type_and_string_args(sub_m);
            let result = QuoteString::quoting_string(QuoteString {}, string.as_str());
            println!("{:?}", result);
        }
        Some(("generate_file", sub_m)) => {
            let (file_path, repeat_times, text) = get_file_path_text_and_numer_of_repetition(sub_m);
            let parsed_number = FileGenerator::get_repeat_times(repeat_times);
            if let Ok(parsed_number) = parsed_number {
                println!("{file_path}, {parsed_number}, {text}");
                FileGenerator::generate_file(FileGenerator{}, file_path.as_str(), parsed_number, text);
            } else {
                println!("{:?}", parsed_number);
            }


        }
        _ => (),
    }
    let end = start.elapsed();
    println!("Time spent = {:.2?}", end);
}
