use regex::Regex;
use crate::{EhResult, Parser, REGEX_MATCH_FAILED};

#[derive(Debug, PartialEq)]
pub struct Thumb {
    pub src: String,
    pub width: u32,
    pub height: u32,
}

impl Parser for Thumb {
    fn parse(doc: &str) -> EhResult<Self> {
        let regex = Regex::new(PATTERN_THUMB).unwrap();
        let captures = regex.captures(doc).ok_or(REGEX_MATCH_FAILED)?;

        let height = captures[1].parse()?;
        let width = captures[2].parse()?;
        let src = String::from(&captures[3]);

        Ok(Thumb {
            src,
            width,
            height,
        })
    }
}

const PATTERN_THUMB: &str = r#"<img[^>]*style="height:(\d+)px;width:(\d+)px[^"]*"[^>]*src="([^"]+)""#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let img = r#"<img style="height:376px;width:250px;top:-18px" alt="[Pixiv] Moca (7010167) [AI Generated]" title="[Pixiv] Moca (7010167) [AI Generated]" src="https://ehgt.org/08/be/08be4188d5b91a484fc7eeb2a952f5d7eeeec5a3-463202-512-768-png_250.jpg">"#;
        assert_eq!(Thumb::parse(img).is_ok(), true);
    }
}
