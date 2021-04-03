use regex::Regex;

pub fn capitalize<T: AsRef<str>>(val: T) -> String {
    let mut is_prev_whitespace = true;

    val.as_ref()
        .chars()
        .map(|c| {
            if c.is_whitespace() {
                is_prev_whitespace = true;

                c.to_string()
            } else if is_prev_whitespace {
                is_prev_whitespace = false;

                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

const WORD_WITH_BOUNDARIES_REGEX: &str = "(\\w+)(\\s*[.?!]*)";

pub fn capitalize_regex<T: AsRef<str>>(val: T) -> String {
    let re = Regex::new(WORD_WITH_BOUNDARIES_REGEX).unwrap();

    let text = val.as_ref();

    let mut result = String::with_capacity(text.len());

    for cap in re.captures_iter(text) {
        let word = &cap[1];
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();

        result.push_str(first_char.to_uppercase().collect::<String>().as_str());
        result.push_str(chars.as_str());

        let boundary = &cap[2];
        result.push_str(boundary);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_empty() {
        assert_eq!(String::new(), capitalize(""))
    }

    #[test]
    fn test_capitalize_one_word() {
        assert_eq!("Abc".to_string(), capitalize("abc"))
    }

    #[test]
    fn test_capitalize_multiple_words() {
        assert_eq!("Hello World!".to_string(), capitalize("hello world!"))
    }

    #[test]
    fn test_capitalize_already_capitalized() {
        assert_eq!("Hello World!".to_string(), capitalize("Hello World!"))
    }

    #[test]
    fn test_capitalize_scharf_ss() {
        assert_eq!("SS".to_string(), capitalize("ß"))
    }

    #[test]
    fn test_capitalize_sentence() {
        assert_eq!(
            "What A Wonderful World".to_string(),
            capitalize("what a wonderful world")
        )
    }

    #[test]
    fn test_capitalize_sentence_with_new_lines() {
        assert_eq!(
            "What\nA\nWonderful\nWorld\n".to_string(),
            capitalize("what\na\nwonderful\nworld\n")
        );

        assert_eq!(
            "What\r\nA\r\nWonderful\r\nWorld\r\n".to_string(),
            capitalize("what\r\na\r\nwonderful\r\nworld\r\n")
        );
    }

    #[test]
    fn test_capitalize_sentence_with_tabs() {
        assert_eq!(
            "What\tA\tWonderful\tWorld".to_string(),
            capitalize("what\ta\twonderful\tworld")
        );
    }

    #[test]
    fn test_capitalize_sentence_with_mixed_whitespaces() {
        assert_eq!(
            "What\tA\tWonderful\nWorld\n".to_string(),
            capitalize("what\ta\twonderful\nworld\n")
        );
    }

    #[test]
    fn test_capitalize_sentence_with_multiple_consecutive_whitespaces() {
        assert_eq!(
            "What\t\tA\t\tWonderful  World\n".to_string(),
            capitalize("what\t\ta\t\twonderful  world\n")
        );
    }

    #[test]
    fn test_capitalize_sentence_with_mixed_consecutive_whitespaces() {
        assert_eq!(
            "What\n\tA\t\nWonderful \r\n World \n   \t  ".to_string(),
            capitalize("what\n\ta\t\nwonderful \r\n world \n   \t  ")
        );
    }

    #[test]
    fn test_capitalize_regex_empty() {
        assert_eq!(String::new(), capitalize_regex(""))
    }

    #[test]
    fn test_capitalize_regex_one_word() {
        assert_eq!("Abc".to_string(), capitalize_regex("abc"))
    }

    #[test]
    fn test_capitalize_regex_multiple_words() {
        assert_eq!("Hello World!".to_string(), capitalize_regex("hello world!"))
    }

    #[test]
    fn test_capitalize_regex_already_capitalize_regexd() {
        assert_eq!("Hello World!".to_string(), capitalize_regex("Hello World!"))
    }

    #[test]
    fn test_capitalize_regex_scharf_ss() {
        assert_eq!("SS".to_string(), capitalize_regex("ß"))
    }

    #[test]
    fn test_capitalize_regex_sentence() {
        assert_eq!(
            "What A Wonderful World".to_string(),
            capitalize_regex("what a wonderful world")
        )
    }

    #[test]
    fn test_capitalize_regex_sentence_with_new_lines() {
        assert_eq!(
            "What\nA\nWonderful\nWorld\n".to_string(),
            capitalize_regex("what\na\nwonderful\nworld\n")
        );

        assert_eq!(
            "What\r\nA\r\nWonderful\r\nWorld\r\n".to_string(),
            capitalize_regex("what\r\na\r\nwonderful\r\nworld\r\n")
        );
    }

    #[test]
    fn test_capitalize_regex_sentence_with_tabs() {
        assert_eq!(
            "What\tA\tWonderful\tWorld".to_string(),
            capitalize_regex("what\ta\twonderful\tworld")
        );
    }

    #[test]
    fn test_capitalize_regex_sentence_with_mixed_whitespaces() {
        assert_eq!(
            "What\tA\tWonderful\nWorld\n".to_string(),
            capitalize_regex("what\ta\twonderful\nworld\n")
        );
    }

    #[test]
    fn test_capitalize_regex_sentence_with_multiple_consecutive_whitespaces() {
        assert_eq!(
            "What\t\tA\t\tWonderful  World\n".to_string(),
            capitalize_regex("what\t\ta\t\twonderful  world\n")
        );
    }

    #[test]
    fn test_capitalize_regex_sentence_with_mixed_consecutive_whitespaces() {
        assert_eq!(
            "What\n\tA\t\nWonderful \r\n World \n   \t  ".to_string(),
            capitalize_regex("what\n\ta\t\nwonderful \r\n world \n   \t  ")
        );
    }
}
