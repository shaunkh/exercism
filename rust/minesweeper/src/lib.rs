pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    for i in 0..minefield.len() {
        let mut s = minefield[i].as_bytes();
        vec.push(String::from(""));

        for j in 0..s.len() {
            println!("{} {}", i, j);
            let mut count = 0;
            //bytes for empty char is 32
            if s[j] == 32 {
                if isInBounds(i + 1, j, minefield) {
                    if checkMine(i + 1, j, minefield) {
                        count += 1;
                    }
                }
                if isInBounds(i, j + 1, minefield) {
                    if checkMine(i, j + 1, minefield) {
                        count += 1;
                    }
                }
                if isInBounds(i + 1, j + 1, minefield) {
                    if checkMine(i + 1, j + 1, minefield) {
                        count += 1;
                    }
                }

                if i as i32 - 1 >= 0 {
                    if isInBounds(i - 1, j, minefield) {
                        if checkMine(i - 1, j, minefield) {
                            count += 1;
                        }
                    }
                    if isInBounds(i - 1, j + 1, minefield) {
                        if checkMine(i - 1, j + 1, minefield) {
                            count += 1;
                        }
                    }

                    if j as i32 - 1 >= 0 {
                        if isInBounds(i - 1, j - 1, minefield) {
                            if checkMine(i - 1, j - 1, minefield) {
                                count += 1;
                            }
                        }
                    }
                }

                if j as i32 - 1 >= 0 {
                    if isInBounds(i, j - 1, minefield) {
                        if checkMine(i, j - 1, minefield) {
                            count += 1;
                        }
                    }
                    if isInBounds(i + 1, j - 1, minefield) {
                        if checkMine(i + 1, j - 1, minefield) {
                            count += 1;
                        }
                    }
                }

                if count == 0 {
                    vec[i].push_str(" ");
                } else {
                    vec[i].push_str(&count.to_string());
                }
            }
            //bytes for * is 42
            else if s[j] == 42 {
                vec[i].push_str("*");
            }
        }
    }

    vec
}

pub fn isInBounds(i: usize, j: usize, minefield: &[&str]) -> bool {
    if i < 0 || i > minefield.len() - 1 {
        return false;
    }

    if j < 0 || j > minefield[i].len() - 1 {
        return false;
    }

    true
}

pub fn checkMine(i: usize, j: usize, minefield: &[&str]) -> bool {
    let s = minefield[i].as_bytes();
    if s[j] == 42 {
        return true;
    }

    false
}
