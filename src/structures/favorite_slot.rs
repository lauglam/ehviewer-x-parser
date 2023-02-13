use regex::Regex;
use crate::{EhResult, Parser, OUT_OF_RANGE, REGEX_MATCH_FAILED};

#[derive(Debug, PartialEq)]
pub struct FavoriteSlot {
    pub r: String,
    pub g: String,
    pub b: String,
    pub value: u32,
}

impl Parser for FavoriteSlot {
    fn parse(doc: &str) -> EhResult<Self> {
        let regex = Regex::new(PATTERN_FAVORITE_SLOT).unwrap();

        let captures = regex.captures(doc).ok_or(REGEX_MATCH_FAILED)?;
        let r = &captures[1];
        let g = &captures[2];
        let b = &captures[3];

        let mut slot = 0;
        for rgb in FAVORITE_SLOT_RGB {
            if r == rgb[0] && g == rgb[1] && b == rgb[2] {
                return Ok(FavoriteSlot {
                    r: String::from(r),
                    g: String::from(g),
                    b: String::from(b),
                    value: slot,
                });
            }

            slot += 1;
        }

        Err(OUT_OF_RANGE)
    }
}

const FAVORITE_SLOT_RGB: [[&str; 3]; 10] = [
    ["0", "0", "0"],
    ["240", "0", "0"],
    ["240", "160", "0"],
    ["208", "208", "0"],
    ["0", "128", "0"],
    ["144", "240", "64"],
    ["64", "176", "240"],
    ["0", "0", "240"],
    ["80", "0", "128"],
    ["224", "128", "224"],
];

const PATTERN_FAVORITE_SLOT: &str = r#"background-color:rgba\((\d+),(\d+),(\d+),"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let style_in_gallery_list = r#"border-color:#000;background-color:rgba(0,0,0,.1)"#;
        assert_eq!(FavoriteSlot::parse(style_in_gallery_list).is_ok(), true);
    }
}
