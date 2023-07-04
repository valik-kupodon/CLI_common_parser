use std::fs::OpenOptions;
use std::io::Write;
use std::num::ParseIntError;

pub struct FileGenerator{}

impl FileGenerator{
    pub fn get_repeat_times(repeat_times: String) -> Result<usize, ParseIntError>{
        let parsed_number = repeat_times.parse()?;
        Ok(parsed_number)
    }
    pub fn generate_file(self, file_path: &str, repeating_number: usize, text: String) {
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
