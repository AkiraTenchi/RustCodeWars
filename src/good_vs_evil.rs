pub fn good_vs_evil(good: &str, evil: &str) -> String {
    let good = good.split(" ");
    let evil = evil.split(" ");
    let good_pow = vec![1, 2, 3, 3, 4, 10];
    let evil_pow = vec![1, 2, 2, 2, 3, 5, 10];
    let mut good_total_pow = 0;
    let mut evil_total_pow = 0;

    for it in good.zip(good_pow) {
        let (g, pow) = it;
        good_total_pow += g.parse::<i32>().unwrap() * pow;
    }

    for it in evil.zip(evil_pow) {
        let (g, pow) = it;
        evil_total_pow += g.parse::<i32>().unwrap() * pow;
    }

    if good_total_pow > evil_total_pow {
        return "Battle Result: Good triumphs over Evil".to_string();
    } else if good_total_pow < evil_total_pow {
        return "Battle Result: Evil eradicates all trace of Good".to_string();
    } else {
        return "Battle Result: No victor on this battle field".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(
            good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"),
            "Battle Result: Good triumphs over Evil"
        );
        assert_eq!(
            good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"),
            "Battle Result: Evil eradicates all trace of Good"
        );
        assert_eq!(
            good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"),
            "Battle Result: No victor on this battle field"
        );
    }
}
