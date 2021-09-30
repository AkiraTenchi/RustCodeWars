use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

struct UrlShortener {
    pub exist_url: HashMap<String, String>,
    pub db: HashMap<String, String>,
    pub s: DefaultHasher,
}

impl UrlShortener {
    fn new() -> Self {
        Self {
            exist_url: HashMap::<String, String>::new(),
            db: HashMap::<String, String>::new(),
            s: DefaultHasher::new(),
        }
    }

    fn shorten(&mut self, long_url: &str) -> String {
        match self.exist_url.get(long_url) {
            Some(url) => return url.clone(),
            None => {
                let mut url = format!("{}{}", "short.ly/", self.compute_hash(long_url));
                while self.db.contains_key(&url) {
                    url = format!(
                        "{}{}",
                        "short.ly/",
                        self.compute_hash(&(long_url.to_owned() + &url))
                    );
                }
                self.exist_url.insert(long_url.to_string(), url.clone());
                self.db.insert(url.clone(), long_url.to_string().clone());
                url
            }
        }
    }

    fn redirect(&self, short_url: &str) -> String {
        self.db.get(short_url).unwrap().clone()
    }

    fn calculate_hash(&mut self, t: &str) -> u64 {
        // let mut s = &self.s;
        t.hash(&mut self.s);
        self.s.finish()
    }

    fn compute_hash(&mut self, s: &str) -> String {
        let mut hash = self.calculate_hash(&s);
        let mut res: String = "".to_string();
        for p in 99..103 {
            let asc = ((hash % p) as u8 % 26) + 97;
            hash = hash / p;
            //if SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() % 2 == 0{
            //    res = String::from_utf8(vec![asc]).unwrap() + &res;
            //} else {
            res += &String::from_utf8(vec![asc]).unwrap();
            //}
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::UrlShortener;
    use crate::assert_valid_short_url;

    #[test]
    fn two_different_urls() {
        let mut url_shortener = UrlShortener::new();

        let short_url_1 =
            url_shortener.shorten("https://www.codewars.com/kata/5ef9ca8b76be6d001d5e1c3e");
        assert_valid_short_url!(&short_url_1);

        let short_url_2 =
            url_shortener.shorten("https://www.codewars.com/kata/5efae11e2d12df00331f91a6");
        assert_valid_short_url!(&short_url_2);

        assert_eq!(
            url_shortener.redirect(&short_url_1),
            "https://www.codewars.com/kata/5ef9ca8b76be6d001d5e1c3e"
        );
        assert_eq!(
            url_shortener.redirect(&short_url_2),
            "https://www.codewars.com/kata/5efae11e2d12df00331f91a6"
        );
    }

    #[test]
    fn same_urls() {
        let mut url_shortener = UrlShortener::new();

        let short_url_1 =
            url_shortener.shorten("https://www.codewars.com/kata/5ef9c85dc41b4e000f9a645f");
        assert_valid_short_url!(&short_url_1);

        let short_url_2 =
            url_shortener.shorten("https://www.codewars.com/kata/5ef9c85dc41b4e000f9a645f");
        assert_valid_short_url!(&short_url_2);

        assert_eq!(
            short_url_1, short_url_2,
            "Should work with the same long URLs"
        );
        assert_eq!(
            url_shortener.redirect(&short_url_1),
            "https://www.codewars.com/kata/5ef9c85dc41b4e000f9a645f",
            "{} should redirect to https://www.codewars.com/kata/5ef9c85dc41b4e000f9a645f",
            &short_url_1,
        );
    }

    #[macro_export]
    macro_rules! assert_valid_short_url {
        ($url:expr) => {
            assert!(
                $url.starts_with("short.ly/"),
                "URL format is incorrect: should start with \"short.ly/\", got: {}",
                $url,
            );

            assert!(
                $url.len() < 14,
                "URL format is incorrect: length should be < 14 characters, got: {}",
                $url,
            );

            // As the URL contains "short.ly/", we can safely index using [9..]
            assert!(
                $url[9..].bytes().all(|b| b.is_ascii_lowercase()),
                "URL format is incorrect: should contain lowercase letters at the end, got: {}",
                $url,
            );
        };
    }
}
