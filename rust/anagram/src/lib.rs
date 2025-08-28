use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    for &possible_anagram in possible_anagrams {
        // if both lengths don't match, continue
        // lowercase them both
        // sort both and check for equality
    }
    anagrams


    // let mut anagrams = HashSet::new();
    //
    // for &possible_anagram in possible_anagrams {
    //     let mut letters_to_check: Vec<char> = possible_anagram.chars().collect();
    //
    //     for char_in_word in word.chars() {
    //         if letters_to_check.contains(&char_in_word) {
    //             if let Some(c) = letters_to_check.iter().position(|&v| v == char_in_word) {
    //                 letters_to_check.remove(c);
    //             }
    //         }
    //     }
    //
    //     if letters_to_check.is_empty() {
    //         anagrams.insert(possible_anagram);
    //     }
    // }
    // anagrams
}
