use regex::Regex;
use crate::{EhResult, Parser, REGEX_MATCH_FAILED};

#[derive(Debug, PartialEq)]
pub struct Rating {
    pub value: f32,
}

impl Parser for Rating {
    fn parse(doc: &str) -> EhResult<Self> {
        let reg = Regex::new(PATTERN_RATING).unwrap();
        let mut n1 = i32::MIN;
        let mut n2 = i32::MIN;

        let mut value = 5 as f32;
        let mut ms = reg.find_iter(doc);
        if let Some(m) = ms.next() {
            n1 = m.as_str().replace("px", "").parse()?;
        }

        if let Some(m) = ms.next() {
            n2 = m.as_str().replace("px", "").parse()?;
        }

        if n1 != i32::MIN && n2 != i32::MIN {
            value -= (n1 / 16) as f32;
            if n2 == 21 {
                value -= 0.5 as f32;
            }

            Ok(Rating { value })
        } else {
            Err(REGEX_MATCH_FAILED)
        }
    }
}

const PATTERN_RATING: &str = r#"\d+px"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rating_test() {
        let rating_style = "background-position:0px -21px;opacity:0.53333333333333";
        assert_eq!(Rating::parse(&rating_style).unwrap().value, 4.5 as f32);
    }
}
