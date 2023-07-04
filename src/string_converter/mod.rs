pub struct QuoteString {}

impl QuoteString {
    pub fn quoting_string(self, string: &str) -> Vec<String> {
        let punctuation_free_string = self.remove_punctuation(string);
        punctuation_free_string
            .split(' ')
            .map(|word| word.to_owned())
            .collect()
    }

    fn remove_punctuation(self, string: &str) -> String {
        string
            .chars()
            .filter(|char| !char.is_ascii_punctuation())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quoting_string() {
        let quote_string = QuoteString {};
        let string = "Hello, World! This is a test string.";
        let result = quote_string.quoting_string(string);
        let expected = vec![
            "Hello".to_string(),
            "World".to_string(),
            "This".to_string(),
            "is".to_string(),
            "a".to_string(),
            "test".to_string(),
            "string".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quoting_string_empty() {
        let quote_string = QuoteString {};
        let string = "";
        let result = quote_string.quoting_string(string);
        let expected: Vec<String> = vec!["".to_owned()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quoting_string_with_punctuation() {
        let quote_string = QuoteString {};
        let string = "Hello, World! This is a test string.";
        let result = quote_string.quoting_string(string);
        let expected = vec![
            "Hello".to_string(),
            "World".to_string(),
            "This".to_string(),
            "is".to_string(),
            "a".to_string(),
            "test".to_string(),
            "string".to_string(),
        ];
        assert_eq!(result, expected);
    }
}
