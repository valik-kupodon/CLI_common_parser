use std::fs::OpenOptions;
use std::io::Write;
use std::num::ParseIntError;
use std::sync::mpsc;
use std::thread;

pub struct FileGenerator{}

impl FileGenerator{
    pub fn get_repeat_times(repeat_times: String) -> Result<usize, ParseIntError>{
        let parsed_number = repeat_times.parse()?;
        Ok(parsed_number)
    }
    pub fn generate_file(repeating_number: usize, text: String) -> Vec<String> {
        let mut content:Vec<String> = Vec::new();
        for _i in 0..repeating_number {
            content.push(text.clone())
        }
        content
    }

    pub fn write_to_file(file_path: &str, content: Vec<String>){
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)
            .expect("Failed to open the file");

        let content_str = content.join("");

        if let Err(err) = file.write_all(content_str.as_bytes()) {
            eprintln!("Failed to append to file: {}", err);
        } else {
            println!("Successfully appended to file {file_path}.");
        }
    }

}
#[derive(Clone, Copy)]
pub struct ThreadFileGeneration{}

impl ThreadFileGeneration {
    pub fn write_file_in_threads(self, repeating_number: usize, text: String, file_path: &str) {
        let (sender, receiver) = mpsc::channel();
        let (first_three_threads, fourth_thread) = self.get_repeating_number_for_threads(&repeating_number);

        for i in 0..4 {
            let sender = sender.clone();
            let mut repeating_times = first_three_threads.clone();
            if i == 4 {
                repeating_times = fourth_thread.clone();
            }
            let text_copy = text.clone();
            thread::spawn(move || {
                let content = FileGenerator::generate_file(repeating_times, text_copy);
                sender.send(content).unwrap();
                println!("Thread {i}, send");
            }
            );

        }
        for vector in receiver.iter().take(4) {
            FileGenerator::write_to_file(file_path, vector)
        }
    }

    fn get_repeating_number_for_threads(self, repeating_number: &usize) -> (usize, usize) {
        let first_three_threads = repeating_number / 4;
        println!("{first_three_threads}, {repeating_number}");
        let fourth_thread = repeating_number - (&first_three_threads * 3);
        println!("{fourth_thread}");
        (first_three_threads, fourth_thread)
    }
}


#[cfg(test)]
mod test {
    // use std::fs::{File, remove_file};
    // use std::io::Read;
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

    // const FILE_PATH: &str = "test_file.txt";

    // #[test]
    // fn test_generate_file() {
    //     let repeating_number = 3;
    //     let text = "Hello, World!";
    //     FileGenerator::generate_file(FILE_PATH, repeating_number, text.to_string());
    //
    //     let mut file = File::open(FILE_PATH).expect("Failed to open the file");
    //     let mut file_content = String::new();
    //     file.read_to_string(&mut file_content)
    //         .expect("Failed to read file content");
    //
    //     let expected_content = format!("{text}{text}{text}");
    //     assert_eq!(file_content, expected_content);
    //     let _ = remove_file(FILE_PATH);
    // }
}
