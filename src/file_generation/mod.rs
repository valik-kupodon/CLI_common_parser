use std::fs::OpenOptions;
use std::io::Write;
use std::num::ParseIntError;
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
pub struct FileGenerator {}

impl FileGenerator {
    pub fn get_repeat_times(repeat_times: String) -> Result<usize, ParseIntError> {
        let parsed_number = repeat_times.parse()?;
        Ok(parsed_number)
    }
    pub fn generate_file(repeating_number: usize, text: String) -> String {
        text.repeat(repeating_number)
    }

    pub fn write_to_file(file_path: &str, content: String) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)
            .expect("Failed to open the file");

        if let Err(err) = file.write_all(content.as_bytes()) {
            eprintln!("Failed to append to file: {:?}", err);
        } else {
            println!("Successfully appended to file {file_path}.");
        }
    }
}
#[derive(Clone, Copy)]
pub struct ThreadFileGeneration {}

impl ThreadFileGeneration {
    pub fn write_file_in_threads(self, repeating_number: usize, text: String, file_path: &str) {
        let (sender, receiver) = mpsc::channel();
        let (first_three_threads, fourth_thread) =
            self.get_repeating_number_for_threads(&repeating_number);

        for i in 0..8 {
            let sender = sender.clone();
            let mut repeating_times = first_three_threads;
            if i == 4 {
                repeating_times = fourth_thread;
            }
            let text_copy = text.clone();
            thread::spawn(move || {
                let content = FileGenerator::generate_file(repeating_times, text_copy);
                sender.send(content).unwrap();
                println!("Thread {i}, send");
            });
        }
        for vector in receiver.iter().take(8) {
            FileGenerator::write_to_file(file_path, vector)
        }
    }

    fn get_repeating_number_for_threads(self, repeating_number: &usize) -> (usize, usize) {
        let first_three_threads = repeating_number / 6;
        println!("{first_three_threads}, {repeating_number}");
        let fourth_thread = repeating_number - (&first_three_threads * 5);
        println!("{fourth_thread}");
        (first_three_threads, fourth_thread)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_repeat_times_success() {
        let repeat_times = FileGenerator::get_repeat_times("10".to_owned());
        assert_eq!(repeat_times, Ok(10))
    }

    #[test]
    fn test_get_repeat_times_not_numeric() {
        let repeat_times = FileGenerator::get_repeat_times("abc".to_owned());
        assert!(repeat_times.is_err())
    }

    #[test]
    fn test_get_repeat_times_negative_int() {
        let repeat_times = FileGenerator::get_repeat_times("-10".to_owned());
        assert!(repeat_times.is_err());
    }

    #[test]
    fn test_generate_file() {
        let repeating_number = 3;
        let text = "Hello, World!";
        let file_content = FileGenerator::generate_file(repeating_number, text.to_owned());
        let expected_content = format!("{text}{text}{text}");
        assert_eq!(file_content, expected_content);
    }
}
