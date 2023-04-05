use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();
    let mut word_map: HashMap<char, u32> = HashMap::new();
    let lowercase_word = word.to_ascii_lowercase();

    for l in 0..lowercase_word.len() {
        let mut count = 0;
        for c in lowercase_word.chars() {
            if c == lowercase_word.chars().nth(l).unwrap() {
                count += 1;
            }
        }
        word_map.insert(lowercase_word.chars().nth(l).unwrap(), count);
    }

    for n in 0..possible_anagrams.len() {
        if lowercase_word != possible_anagrams[n].to_ascii_lowercase() {
            let mut anagram_map: HashMap<char, u32> = HashMap::new();
            for l in 0..possible_anagrams[n].len() {
                let mut count = 0;
                for c in possible_anagrams[n].chars() {
                    if c.to_ascii_lowercase()
                        == possible_anagrams[n]
                            .chars()
                            .nth(l)
                            .unwrap()
                            .to_ascii_lowercase()
                    {
                        count += 1;
                    }
                }
                anagram_map.insert(
                    possible_anagrams[n]
                        .chars()
                        .nth(l)
                        .unwrap()
                        .to_ascii_lowercase(),
                    count,
                );
            }
            println!("{:?}", anagram_map);
            println!("{:?}", anagram_map);
            if anagram_map == word_map {
                result.insert(possible_anagrams[n]);
            }
        }
    }

    result
}
