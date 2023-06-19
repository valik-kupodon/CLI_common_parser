use crate::command_line_interface::get_command_type_and_string_args;
use std::time::Instant;
use crate::string_converter::QuoteString;

mod command_line_interface;
mod string_converter;

fn main() {
    let start = Instant::now();
    let matches = command_line_interface::register_args();
    // Matches the commands and performs actions
    if let Some(("run", sub_m)) = matches.subcommand() {
        let (_, string) = get_command_type_and_string_args(sub_m);
        let result = QuoteString::quoting_string(QuoteString {}, string.as_str());
        println!("{:?}", result);
    }
    let end = start.elapsed();
    println!("Time spent = {:.2?}", end);
}
