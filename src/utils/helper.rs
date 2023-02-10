use std::borrow::Cow;
use crate::EhResult;

#[inline]
pub fn unescape(str: &str) -> Cow<str> {
    const ESCAPE_CHARACTER_LIST: [&str; 7] = ["&amp;", "&lt;", "&gt;", "&quot;", "&#039;", "&times;", "&nbsp;"];
    const UNESCAPE_CHARACTER_LIST: [&str; 7] = ["&", "<", ">", "\"", "'", "×", " "];

    let mut result = Cow::Borrowed(str);
    let idx_opt = ESCAPE_CHARACTER_LIST.iter()
        .position(|&escape| str.contains(escape));

    if let Some(idx) = idx_opt {
        result = Cow::Owned(result.replace(ESCAPE_CHARACTER_LIST[idx], UNESCAPE_CHARACTER_LIST[idx]));
    }

    result
}

#[inline]
pub fn trim(str: &str) -> Cow<str> {
    match unescape(str) {
        Cow::Borrowed(str) => Cow::Borrowed(str.trim()),
        Cow::Owned(str) => Cow::Owned(String::from(str.trim())),
    }
}

#[inline]
pub fn parse_u32(str: &str) -> EhResult<u32> {
    Ok(trim(str).parse::<u32>()?)
}

#[inline]
pub fn parse_i32(str: &str) -> EhResult<i32> {
    Ok(trim(str).parse::<i32>()?)
}

#[inline]
pub fn parse_u64(str: &str) -> EhResult<u64> {
    Ok(trim(str).parse::<u64>()?)
}

#[inline]
pub fn parse_f32(str: &str) -> EhResult<f32> {
    Ok(trim(str).parse::<f32>()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn unescape_test() {
        let r = generate_range();
        let input = &ESCAPE_CHARACTER_LIST[r.0..r.1].join("");

        assert_eq!(&unescape(input), &UNESCAPE_CHARACTER_LIST[r.0..r.1].join(""));
    }

    #[test]
    fn trim_test() {
        let r = generate_range();
        let input = &format!("{} ", ESCAPE_CHARACTER_LIST[r.0..r.1].join(""));

        assert_eq!(&trim(input), &UNESCAPE_CHARACTER_LIST[r.0..r.1].join(""));
    }

    fn generate_range() -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let start = rng.gen_range(0..6);
        let end = rng.gen_range(start..7);

        (start, end)
    }

    const ESCAPE_CHARACTER_LIST: [&str; 7] = ["&amp;", "&lt;", "&gt;", "&quot;", "&#039;", "&times;", "&nbsp;"];
    const UNESCAPE_CHARACTER_LIST: [&str; 7] = ["&", "<", ">", "\"", "'", "×", " "];
}
