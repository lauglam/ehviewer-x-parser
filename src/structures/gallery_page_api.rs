use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::utils::unescape;

#[derive(Debug, PartialEq)]
pub struct GalleryPageApi {
    pub image_url: String,
    pub skip_hath_key: String,
    pub origin_image_url: String,
}

impl GalleryPageApi {
    pub fn parse(json: &str) -> Result<GalleryPageApi, String> {
        parse_internal(json).ok_or(String::from("parses gallery page api fail."))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GalleryPageApiInternal {
    i3: String,
    i6: String,
    i7: String,
}

fn parse_internal(json: &str) -> Option<GalleryPageApi> {
    if let Ok(internal) = serde_json::from_str::<GalleryPageApiInternal>(json) {
        const PATTERN_IMAGE_URL: &str = r#"<img[^>]*src="([^"]+)" style"#;
        const PATTERN_SKIP_HATH_KEY: &str = r#"onclick="return nl\('([^\)]+)'\)"#;
        const PATTERN_ORIGIN_IMAGE_URL: &str = r#"<a href="([^"]+)fullimg.php([^"]+)">"#;

        let regex = Regex::new(PATTERN_IMAGE_URL).unwrap();
        let captures = regex.captures(&internal.i3)?;
        let image_url = String::from(&captures[1]);

        let regex = Regex::new(PATTERN_SKIP_HATH_KEY).unwrap();
        let captures = regex.captures(&internal.i6)?;
        let skip_hath_key = String::from(&captures[1]);

        let regex = Regex::new(PATTERN_ORIGIN_IMAGE_URL).unwrap();
        let captures = regex.captures(&internal.i7)?;
        let origin_image_url = format!("{}{}{}", &captures[1], r#"fullimg.php"#, unescape(&captures[2]));

        return Some(GalleryPageApi {
            image_url,
            skip_hath_key,
            origin_image_url,
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {}
}
