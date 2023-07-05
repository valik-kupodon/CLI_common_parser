use std::fs::OpenOptions;
use std::io::Write;
use std::num::ParseIntError;

pub struct FileGenerator{}

impl FileGenerator{
    pub fn get_repeat_times(repeat_times: String) -> Result<usize, ParseIntError>{
        let parsed_number = repeat_times.parse()?;
        Ok(parsed_number)
    }
    pub fn generate_file(file_path: &str, repeating_number: usize, text: String) {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)
            .expect("Failed to open the file");
        for _i in 0..repeating_number {
            if let Err(err) = file.write_all(text.as_bytes()) {
                eprintln!("Failed to append to file: {}", err);
            } else {
                println!("Successfully appended to file {file_path}.");
            }
        }
    }

}


#[cfg(test)]
mod test {
    use std::fs::{File, remove_file};
    use std::io::Read;
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

    const FILE_PATH: &str = "test_file.txt";

    #[test]
    fn test_generate_file() {
        let repeating_number = 3;
        let text = "Hello, World!";
        FileGenerator::generate_file(FILE_PATH, repeating_number, text.to_string());

        let mut file = File::open(FILE_PATH).expect("Failed to open the file");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("Failed to read file content");

        let expected_content = format!("{text}{text}{text}");
        assert_eq!(file_content, expected_content);
        let _ = remove_file(FILE_PATH);
    }
}
