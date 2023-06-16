use std::time::Instant;
use crate::command_line_interface::get_command_type_and_string_args;

mod command_line_interface;

fn main() {
    let start = Instant::now();
    let matches = command_line_interface::register_args();
    // Matches the commands and performs actions
    if let Some(("run", sub_m)) = matches.subcommand() {
        let (command_type, string) = get_command_type_and_string_args(sub_m);
        println!("{command_type}, {string}");
    }
    let end = start.elapsed();
    println!("Time spent = {:.2?}", end);
}
