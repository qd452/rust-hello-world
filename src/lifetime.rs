// Title: Lifetime
// More: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
// https://chat.openai.com/share/43dbd157-83c3-4b5c-990d-8e8c52709988

#![allow(dead_code)]
struct Excerpt<'a> {
    part: &'a str,
}

#[allow(dead_code)]
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        let sentence = String::from("Hello, world!");
        assert_eq!(first_word(&sentence), "Hello,");
    }

    #[test]
    fn test_first_word_empty_string() {
        let sentence = String::from("");
        assert_eq!(first_word(&sentence), "");
    }

    #[test]
    fn test_first_word_no_space() {
        let sentence = String::from("Hello");
        assert_eq!(first_word(&sentence), "Hello");
    }

    #[test]
    fn test_first_word_multiple_spaces() {
        let sentence = String::from("Hello   world!");
        assert_eq!(first_word(&sentence), "Hello");
    }

    #[test]
    fn test_lifetime() {
        let sentence = String::from("Hello, world!");

        let word;
        {
            let excerpt = Excerpt {
                part: first_word(&sentence),
            };
            word = excerpt.part;
        }

        println!("The first word is: {}", word);
        assert_eq!(word, "Hello,");
    }
}
