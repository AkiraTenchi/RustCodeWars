

pub fn buddy(start: i64, limit: i64) -> Option<(i64, i64)> {
    for i in start..limit {
        let div_sum = get_divisors_sum(i);
        dbg!(i, div_sum);
        if (div_sum - 1) > i {
            if i == get_divisors_sum(div_sum) {
                return Some((i, div_sum));
            }
        }
    }
    return None;
}

// fn get_divisors_sum(n: i64) -> i64 {
//     (1..n).into_iter().filter(|&x| n % x == 0).sum()
// }

fn get_divisors_sum(n:i64) -> i64 {
    let mut sum = 0;
    let target = (n as f32).sqrt().round() as i64;

    for i in 2..target {
        if n % i == 0 {
            sum += i;
            if i != n / i {
                sum += n / i;
            }
        }
    }

    return sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(start: i64, limit: i64, exp: Option<(i64, i64)>) -> () {
        println!("start:{}", start);
        println!("limit:{}", limit);
        let ans = buddy(start, limit);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(10, 76, Some((48, 75)));
        dotest(1081180, 1103735, Some((1081184, 1331967)));
    }
}
