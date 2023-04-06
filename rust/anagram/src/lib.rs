use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();
    let lowercase_word = word.to_lowercase();
    let sorted_word = get_sorted(&lowercase_word);

    for possible_anagram in possible_anagrams {
        let lowercase_anagram = possible_anagram.to_lowercase();
        if lowercase_word == lowercase_anagram {
            continue;
        }

        let sorted_anagram = get_sorted(&lowercase_anagram);
        if sorted_word == sorted_anagram {
            result.insert(possible_anagram);
        }
    }
    result
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}
