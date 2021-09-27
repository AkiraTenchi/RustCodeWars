pub fn revrot(s: &str, c: usize) -> String {
    if s.is_empty() || s.len() <= c || c <= 0{
        return "".to_string();
    }

    let mut str : String = s.clone().to_string();
    let mut res : String = "".to_string();

    while str.len() >= c {
        let chunk : String = str.chars().into_iter().take(c).collect();
        str.replace_range(..c, "");
        println!("chunk: {}, s: {}, res: {}", chunk, str, res);
        let number : u32 = chunk.chars().map(|c| c.to_digit(10).unwrap()).map(|x| x * x).sum(); 
        if number % 2 == 0{
            res = format!("{}{}", res, (chunk.chars().rev().collect::<String>()));
            println!{"test: {}", chunk.chars().rev().collect::<String>()}
        } else {
            res = format!("{}{}{}", res, &chunk[1..c], &chunk[..1])
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::revrot;

    #[test]
    fn testing() {
        assert_eq!(revrot("1234", 0), "");
        assert_eq!(revrot("", 0), "");
        assert_eq!(revrot("1234", 5), "");
        assert_eq!(revrot("733049910872815764", 5), "330479108928157");
        assert_eq!(
            revrot("73304991087281576455176044327690580265896", 8),
            "1994033775182780067155464327690480265895"
        );
    }
}
