pub fn capitalize<T: AsRef<str>>(val: T) -> String {
    val.as_ref()
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
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
}
