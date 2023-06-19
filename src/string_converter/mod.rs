pub struct QuoteString{}

impl QuoteString{
    pub fn quoting_string(self, string: &str) -> Vec<String>{
        let punctuation_free_string = self.remove_punctuation(string);
        punctuation_free_string.split(' ')
            .map(|word| word.to_owned())
            .collect()
    }

    fn remove_punctuation(self, string: &str) -> String{
        string.chars()
            .filter(|char| !char.is_ascii_punctuation())
            .collect()
    }
}
