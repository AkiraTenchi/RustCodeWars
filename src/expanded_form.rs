pub fn expanded_form(n: u64) -> String {
    let mut res: String = "".to_string();
    let mut num = n.clone();
    let mut modu = 10;
    let mut remain_exists = false;

    while num > modu {
        let remainder = num % modu;
        if remainder != 0 {
            remain_exists = true;
            if res != "" {
                res = format!("{} + {}", remainder, res);
            } else if res == "" {
                res = remainder.to_string();
            }
        }
        num -= remainder;
        modu *= 10;
    }

    if remain_exists {
        return format!("{} + {}", num, res);
    }
    num.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(expanded_form(2), "2");
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }
}
