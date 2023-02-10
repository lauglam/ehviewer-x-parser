use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::{EhResult, ParseError, Parser, utils::unescape};

#[derive(Debug, PartialEq)]
pub struct GalleryPageApi {
    pub image_url: String,
    pub skip_hath_key: String,
    pub origin_image_url: String,
}

impl Parser for GalleryPageApi {
    fn parse(doc: &str) -> EhResult<Self> {
        let internal = serde_json::from_str::<GalleryPageApiInternal>(doc)?;

        let regex = Regex::new(PATTERN_IMAGE_URL).unwrap();
        let captures = regex.captures(&internal.i3).ok_or(ParseError::RegexMatchFailed)?;
        let image_url = String::from(&captures[1]);

        let regex = Regex::new(PATTERN_SKIP_HATH_KEY).unwrap();
        let captures = regex.captures(&internal.i6).ok_or(ParseError::RegexMatchFailed)?;
        let skip_hath_key = String::from(&captures[1]);

        let regex = Regex::new(PATTERN_ORIGIN_IMAGE_URL).unwrap();
        let captures = regex.captures(&internal.i7).ok_or(ParseError::RegexMatchFailed)?;
        let origin_image_url = format!("{}{}{}", &captures[1], r#"fullimg.php"#, unescape(&captures[2]));

        Ok(GalleryPageApi {
            image_url,
            skip_hath_key,
            origin_image_url,
        })
    }
}

const PATTERN_IMAGE_URL: &str = r#"<img[^>]*src="([^"]+)" style"#;
const PATTERN_SKIP_HATH_KEY: &str = r#"onclick="return nl\('([^\)]+)'\)"#;
const PATTERN_ORIGIN_IMAGE_URL: &str = r#"<a href="([^"]+)fullimg.php([^"]+)">"#;

#[derive(Debug, Serialize, Deserialize)]
struct GalleryPageApiInternal {
    i3: String,
    i6: String,
    i7: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {}
}
