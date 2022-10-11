const WORD_LENGTH: usize = 5;

pub fn sanitize_word(word: &str) -> String {
    word.trim()
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}

pub fn is_valid_word(word: &str) -> bool {
    return word.len() == WORD_LENGTH;
}

pub fn get_words(words: &str) -> Vec<String> {
    words
        .split('\n')
        .map(sanitize_word)
        .filter(|l| is_valid_word(l))
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sanitze_word() {
        assert_eq!(sanitize_word("HELLO"), "HELLO");
        assert_eq!(sanitize_word("  HELLO"), "HELLO");
        assert_eq!(sanitize_word("HELLO\n"), "HELLO");
        assert_eq!(sanitize_word("HELLO  "), "HELLO");
        assert_eq!(sanitize_word("HEL  LO"), "HELLO");
        assert_eq!(sanitize_word("H3L\nL0"), "HLL");
    }

    #[test]
    fn test_is_valid_word() {
        assert_eq!(is_valid_word("HELLO"), true);
        assert_eq!(is_valid_word("WORLD"), true);
        assert_eq!(is_valid_word("WRLD"), false);
        assert_eq!(is_valid_word("WOORLD"), false);
    }
    #[test]
    fn test_get_words() {
        assert_eq!(get_words("HELLO\nWORLD"), ["HELLO", "WORLD"]);
        assert_eq!(get_words("HELLO\n\nWORLD"), ["HELLO", "WORLD"]);
        assert_eq!(get_words("HEY\nWORLD"), ["WORLD"]);
        let empty_vec: Vec<String> = Vec::new();
        assert_eq!(get_words("HEY\nYOU\nPIKACHU"), empty_vec);
    }
}
