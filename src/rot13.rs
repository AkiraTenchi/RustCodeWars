pub fn rot13(message: &str) -> String {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let alphabet_upper: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut res = "".to_string();
    for c in message.chars() {
        if alphabet.contains(&c) {
            res = format!("{}{}", res, shift13(c, &alphabet));
        } else if alphabet_upper.contains(&c) {
            res = format!("{}{}", res, shift13(c, &alphabet_upper));
        } else {
            res = format!("{}{}", res, c);
        }
    }
    res
}

fn shift13(c: char, alp: &Vec<char>) -> char {
    let index = alp.iter().position(|&r| r == c).unwrap();
    if index + 13 >= alp.len() {
        return *alp.get((index + 13) - alp.len()).unwrap();
    } else {
        return *alp.get(index + 13).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
    }
}
