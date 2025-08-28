use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    for &possible_anagram in possible_anagrams {
        if possible_anagram.len() != word.len() { continue }

        if possible_anagram.to_lowercase() == word.to_lowercase() { continue }

        let mut possible_anagram_vec: Vec<char> = possible_anagram.to_lowercase().chars().collect();
        possible_anagram_vec.sort();

        let mut word_vec: Vec<char> = word.to_lowercase().chars().collect();
        word_vec.sort();

        if possible_anagram_vec == word_vec {
            anagrams.insert(possible_anagram);
        }
    }
    anagrams
}
