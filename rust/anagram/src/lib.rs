use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    let result: HashSet<&str> = HashSet::new();
    for n in 0..possible_anagrams.len() {
        if word != possible_anagrams[n] {
            break;
        }
        println!("{}: {}", n, possible_anagrams[n])
    }
    result
}
