use std::borrow::Cow;

const ESCAPE_CHARACTER_LIST: [&str; 7] = ["&amp;", "&lt;", "&gt;", "&quot;", "&#039;", "&times;", "&nbsp;"];
const UNESCAPE_CHARACTER_LIST: [&str; 7] = ["&", "<", ">", "\"", "'", "×", " "];

#[inline]
pub fn unescape_xml(str: &str) -> Cow<str> {
    let mut result = Cow::Borrowed(str);
    for index in 0..ESCAPE_CHARACTER_LIST.len() {
        if str.contains(ESCAPE_CHARACTER_LIST[index]) {
            result = Cow::Owned(result.replace(ESCAPE_CHARACTER_LIST[index], UNESCAPE_CHARACTER_LIST[index]));
        }
    }

    result
}

#[inline]
pub fn trim(str: &str) -> Cow<str> {
    match unescape_xml(str) {
        Cow::Borrowed(str) => Cow::Borrowed(str.trim()),
        Cow::Owned(str) => Cow::Owned(String::from(str.trim())),
    }
}

#[inline]
pub fn parse_usize(str: &str, or_else: usize) -> usize {
    trim(str).parse::<usize>().unwrap_or(or_else)
}

#[inline]
pub fn parse_isize(str: &str, or_else: isize) -> isize {
    trim(str).parse::<isize>().unwrap_or(or_else)
}

#[inline]
pub fn parse_i64(str: &str, or_else: i64) -> i64 {
    trim(str).parse::<i64>().unwrap_or(or_else)
}

#[inline]
pub fn parse_f32(str: &str, or_else: f32) -> f32 {
    trim(str).parse::<f32>().unwrap_or(or_else)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn unescape_xml_test() {
        let r = generate_range();
        let input = &ESCAPE_CHARACTER_LIST[r.0..r.1].join("");
        let expect = &UNESCAPE_CHARACTER_LIST[r.0..r.1].join("");

        assert_eq!(&unescape_xml(input), expect);
    }

    #[test]
    fn trim_test() {
        let r = generate_range();
        let input = &format!("{} ", ESCAPE_CHARACTER_LIST[r.0..r.1].join(""));
        let expect = &UNESCAPE_CHARACTER_LIST[r.0..r.1].join("");

        assert_eq!(&trim(input), expect);
    }

    fn generate_range() -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let start = rng.gen_range(0..6);
        let end = rng.gen_range(start..7);

        (start, end)
    }
}
