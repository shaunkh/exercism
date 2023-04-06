pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for i in 0..minefield.len() {
        let mut s = minefield[i].as_bytes();
        for j in 0..s.len() {
            let mut count = 0;
            //bytes for empty char is 32
            if (s[j] == 32) {}

            //bytes for * is 42
            if (s[j] == 42) {}
        }
    }

    vec
}
