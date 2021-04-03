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

    const THE_RAVEN: &str = "Once upon a midnight dreary, while I pondered, weak and weary,
Over many a quaint and curious volume of forgotten lore—
    While I nodded, nearly napping, suddenly there came a tapping,
As of some one gently rapping, rapping at my chamber door.
“’Tis some visitor,” I muttered, “tapping at my chamber door—
            Only this and nothing more.”

    Ah, distinctly I remember it was in the bleak December;
And each separate dying ember wrought its ghost upon the floor.
    Eagerly I wished the morrow;—vainly I had sought to borrow
    From my books surcease of sorrow—sorrow for the lost Lenore—
For the rare and radiant maiden whom the angels name Lenore—
            Nameless here for evermore.

    And the silken, sad, uncertain rustling of each purple curtain
Thrilled me—filled me with fantastic terrors never felt before;
    So that now, to still the beating of my heart, I stood repeating
    “’Tis some visitor entreating entrance at my chamber door—
Some late visitor entreating entrance at my chamber door;—
            This it is and nothing more.”

    Presently my soul grew stronger; hesitating then no longer,
“Sir,” said I, “or Madam, truly your forgiveness I implore;
    But the fact is I was napping, and so gently you came rapping,
    And so faintly you came tapping, tapping at my chamber door,
That I scarce was sure I heard you”—here I opened wide the door;—
            Darkness there and nothing more.

    Deep into that darkness peering, long I stood there wondering, fearing,
Doubting, dreaming dreams no mortal ever dared to dream before;
    But the silence was unbroken, and the stillness gave no token,
    And the only word there spoken was the whispered word, “Lenore?”
This I whispered, and an echo murmured back the word, “Lenore!”—
            Merely this and nothing more.

    Back into the chamber turning, all my soul within me burning,
Soon again I heard a tapping somewhat louder than before.
    “Surely,” said I, “surely that is something at my window lattice;
      Let me see, then, what thereat is, and this mystery explore—
Let my heart be still a moment and this mystery explore;—
            ’Tis the wind and nothing more!”

    Open here I flung the shutter, when, with many a flirt and flutter,
In there stepped a stately Raven of the saintly days of yore;
    Not the least obeisance made he; not a minute stopped or stayed he;
    But, with mien of lord or lady, perched above my chamber door—
Perched upon a bust of Pallas just above my chamber door—
            Perched, and sat, and nothing more.

Then this ebony bird beguiling my sad fancy into smiling,
By the grave and stern decorum of the countenance it wore,
“Though thy crest be shorn and shaven, thou,” I said, “art sure no craven,
Ghastly grim and ancient Raven wandering from the Nightly shore—
Tell me what thy lordly name is on the Night’s Plutonian shore!”
            Quoth the Raven “Nevermore.”

    Much I marvelled this ungainly fowl to hear discourse so plainly,
Though its answer little meaning—little relevancy bore;
    For we cannot help agreeing that no living human being
    Ever yet was blessed with seeing bird above his chamber door—
Bird or beast upon the sculptured bust above his chamber door,
            With such name as “Nevermore.”

    But the Raven, sitting lonely on the placid bust, spoke only
That one word, as if his soul in that one word he did outpour.
    Nothing farther then he uttered—not a feather then he fluttered—
    Till I scarcely more than muttered “Other friends have flown before—
On the morrow he will leave me, as my Hopes have flown before.”
            Then the bird said “Nevermore.”

    Startled at the stillness broken by reply so aptly spoken,
“Doubtless,” said I, “what it utters is its only stock and store
    Caught from some unhappy master whom unmerciful Disaster
    Followed fast and followed faster till his songs one burden bore—
Till the dirges of his Hope that melancholy burden bore
            Of ‘Never—nevermore’.”

    But the Raven still beguiling all my fancy into smiling,
Straight I wheeled a cushioned seat in front of bird, and bust and door;
    Then, upon the velvet sinking, I betook myself to linking
    Fancy unto fancy, thinking what this ominous bird of yore—
What this grim, ungainly, ghastly, gaunt, and ominous bird of yore
            Meant in croaking “Nevermore.”

    This I sat engaged in guessing, but no syllable expressing
To the fowl whose fiery eyes now burned into my bosom’s core;
    This and more I sat divining, with my head at ease reclining
    On the cushion’s velvet lining that the lamp-light gloated o’er,
But whose velvet-violet lining with the lamp-light gloating o’er,
            She shall press, ah, nevermore!

    Then, methought, the air grew denser, perfumed from an unseen censer
Swung by Seraphim whose foot-falls tinkled on the tufted floor.
    “Wretch,” I cried, “thy God hath lent thee—by these angels he hath sent thee
    Respite—respite and nepenthe from thy memories of Lenore;
Quaff, oh quaff this kind nepenthe and forget this lost Lenore!”
            Quoth the Raven “Nevermore.”

    “Prophet!” said I, “thing of evil!—prophet still, if bird or devil!—
Whether Tempter sent, or whether tempest tossed thee here ashore,
    Desolate yet all undaunted, on this desert land enchanted—
    On this home by Horror haunted—tell me truly, I implore—
Is there—is there balm in Gilead?—tell me—tell me, I implore!”
            Quoth the Raven “Nevermore.”

    “Prophet!” said I, “thing of evil!—prophet still, if bird or devil!
By that Heaven that bends above us—by that God we both adore—
    Tell this soul with sorrow laden if, within the distant Aidenn,
    It shall clasp a sainted maiden whom the angels name Lenore—
Clasp a rare and radiant maiden whom the angels name Lenore.”
            Quoth the Raven “Nevermore.”

    “Be that word our sign of parting, bird or fiend!” I shrieked, upstarting—
“Get thee back into the tempest and the Night’s Plutonian shore!
    Leave no black plume as a token of that lie thy soul hath spoken!
    Leave my loneliness unbroken!—quit the bust above my door!
Take thy beak from out my heart, and take thy form from off my door!”
            Quoth the Raven “Nevermore.”

    And the Raven, never flitting, still is sitting, still is sitting
On the pallid bust of Pallas just above my chamber door;
    And his eyes have all the seeming of a demon’s that is dreaming,
    And the lamp-light o’er him streaming throws his shadow on the floor;
And my soul from out that shadow that lies floating on the floor
            Shall be lifted—nevermore!";

    #[bench]
    fn bench_capitalize_the_raven(b: &mut Bencher) {
        b.iter(|| capitalize(THE_RAVEN));
    }

    #[bench]
    fn bench_capitalize_regex_the_raven(b: &mut Bencher) {
        b.iter(|| capitalize_regex(THE_RAVEN));
    }
}
