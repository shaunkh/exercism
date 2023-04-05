use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();
    let mut word_map: HashMap<char, u32> = HashMap::new();

    for l in 0..word.len() {
        let mut count = 0;
        for c in word.chars() {
            if c.to_ascii_lowercase() == word.chars().nth(l).unwrap().to_ascii_lowercase() {
                count += 1;
            }
        }
        word_map.insert(word.chars().nth(l).unwrap(), count);
    }

    for n in 0..possible_anagrams.len() {
        if word != possible_anagrams[n] {
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
                anagram_map.insert(possible_anagrams[n].chars().nth(l).unwrap(), count);
            }
            if anagram_map == word_map {
                result.insert(possible_anagrams[n]);
            }
        }
    }

    result
}
