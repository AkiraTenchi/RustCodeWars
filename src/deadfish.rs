pub fn parse(code: &str) -> Vec<i32> {
    let mut res = Vec::<i32>::new();
    let mut val = 0;
    for c in code.chars() {
        match c {
            'i' => val += 1,
            'd' => val -= 1,
            's' => val *= val,
            'o' => res.push(val),
            _ => {}
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
    }
}
