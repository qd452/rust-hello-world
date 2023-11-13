// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

#[allow(dead_code)]
fn slices_learning() {
    let s = String::from("hellow world");
    let world = first_word(&s);

    // s.clear(); -> this is not work any way

    dbg!(world);
}

#[allow(dead_code)]
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

#[allow(dead_code)]
fn slices_learning2() {
    let mut s = String::from("hellow world");
    let world = first_word_idx(&s);

    s.clear();

    dbg!(world);
}

#[allow(dead_code)]
fn first_word_idx(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        let s = String::from("hello world");
        assert_eq!(first_word(&s), "hello");
    }

    #[test]
    fn test_first_word_empty_string() {
        let s = String::from("");
        assert_eq!(first_word(&s), "");
    }

    #[test]
    fn test_first_word_one_word() {
        let s = String::from("hello");
        assert_eq!(first_word(&s), "hello");
    }

    #[test]
    fn test_first_word_idx() {
        let s = String::from("hello world");
        assert_eq!(first_word_idx(&s), 5);
    }

    #[test]
    fn test_first_word_idx_empty_string() {
        let s = String::from("");
        assert_eq!(first_word_idx(&s), 0);
    }

    #[test]
    fn test_first_word_idx_one_word() {
        let s = String::from("hello");
        assert_eq!(first_word_idx(&s), 5);
    }
}
