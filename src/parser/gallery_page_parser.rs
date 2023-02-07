use regex::Regex;
use crate::utils::unescape;

#[derive(Debug, PartialEq)]
pub struct GalleryPage {
    pub image_url: String,
    pub skip_hath_key: String,
    pub origin_image_url: String,
    pub show_key: String,
}

impl GalleryPage {
    pub fn parse(document: &str) -> Result<GalleryPage, String> {
        if let Some(gallery_page) = parse_internal(document) {
            Ok(gallery_page)
        } else {
            Err(String::from("parses gallery page fail."))
        }
    }
}

fn parse_internal(document: &str) -> Option<GalleryPage> {
    const PATTERN_IMAGE_URL: &str = r#"<img[^>]*src="([^"]+)" style"#;
    const PATTERN_SKIP_HATH_KEY: &str = r#"onclick="return nl\('([^\)]+)'\)"#;
    const PATTERN_ORIGIN_IMAGE_URL: &str = r#"<a href=\"([^\"]+)fullimg.php([^\"]+)\">"#;
    // TODO Not sure about the size of show keys
    const PATTERN_SHOW_KEY: &str = r#"var showkey="([0-9a-z]+)";"#;

    let regex = Regex::new(PATTERN_IMAGE_URL).unwrap();
    let captures = regex.captures(document)?;
    let image_url = String::from(&captures[1]);

    let regex = Regex::new(PATTERN_SKIP_HATH_KEY).unwrap();
    let captures = regex.captures(document)?;
    let skip_hath_key = String::from(&captures[1]);

    let regex = Regex::new(PATTERN_ORIGIN_IMAGE_URL).unwrap();
    let captures = regex.captures(document)?;
    let origin_image_url = format!("{}{}{}", &captures[1], r#"fullimg.php"#, unescape(&captures[2]));

    let regex = Regex::new(PATTERN_SHOW_KEY).unwrap();
    let captures = regex.captures(document)?;
    let show_key = String::from(&captures[1]);

    Some(GalleryPage {
        image_url,
        skip_hath_key,
        origin_image_url,
        show_key,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {}
}
