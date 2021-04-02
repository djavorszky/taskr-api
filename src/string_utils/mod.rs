pub fn capitalize<T: AsRef<str>>(val: T) -> String {
    let mut previous_whitespace = true;

    val.as_ref()
        .chars()
        .map(|c| {
            if c.is_whitespace() {
                previous_whitespace = true;

                c.to_string()
            } else if previous_whitespace == true {
                previous_whitespace = false;

                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
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
}
