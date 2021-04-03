#![allow(soft_unstable)]

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
    lazy_static! {
        static ref RE: Regex = Regex::new(WORD_WITH_BOUNDARIES_REGEX).unwrap();
    }

    let text = val.as_ref();

    let mut result = String::with_capacity(text.len());

    for cap in RE.captures_iter(text) {
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
    extern crate test;

    use super::*;
    use test::Bencher;

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

    #[bench]
    fn bench_capitalize(b: &mut Bencher) {
        let test_sentence = "what a wonderful world!";

        b.iter(|| capitalize(test_sentence));
    }

    #[bench]
    fn bench_capitalize_regex(b: &mut Bencher) {
        let test_sentence = "what a wonderful world!";

        b.iter(|| capitalize_regex(test_sentence));
    }

    const POEM: &str = "Do not go gentle into that good night,
Old age should burn and rave at close of day;
Rage, rage against the dying of the light.

Though wise men at their end know dark is right,
Because their words had forked no lightning they
Do not go gentle into that good night.

Good men, the last wave by, crying how bright
Their frail deeds might have danced in a green bay,
Rage, rage against the dying of the light.

Wild men who caught and sang the sun in flight,
And learn, too late, they grieved it on its way,
Do not go gentle into that good night.

Grave men, near death, who see with blinding sight
Blind eyes could blaze like meteors and be gay,
Rage, rage against the dying of the light.

And you, my father, there on the sad height,
Curse, bless, me now with your fierce tears, I pray.
Do not go gentle into that good night.
Rage, rage against the dying of the light.";

    #[bench]
    fn bench_capitalize_poem(b: &mut Bencher) {
        b.iter(|| capitalize(POEM));
    }

    #[bench]
    fn bench_capitalize_regex_poem(b: &mut Bencher) {
        b.iter(|| capitalize_regex(POEM));
    }
}
