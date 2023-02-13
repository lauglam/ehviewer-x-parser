use regex::Regex;
use crate::{const_concat, eh_url, EhResult, Parser, REGEX_MATCH_FAILED};

#[derive(Debug, PartialEq)]
pub struct GalleryPageUrl {
    pub gid: u64,
    pub p_token: String,
    pub page: u32,
}

impl Parser for GalleryPageUrl {
    /// ```text
    /// https://e-hentai.org/s/35142216f7/2062874-16
    /// ```
    fn parse(doc: &str) -> EhResult<Self> {
        let regex = Regex::new(URL_STRICT_PATTERN).unwrap();
        let captures = regex.captures(doc).ok_or(REGEX_MATCH_FAILED)?;
        let p_token = String::from(&captures[1]);
        let gid = captures[2].parse()?;
        let page = captures[3].parse::<u32>()? - 1;

        Ok(GalleryPageUrl {
            gid,
            p_token,
            page,
        })
    }
}

const URL_STRICT_PATTERN: &str = const_concat!("https?://(?:", eh_url::DOMAIN_EX, "|", eh_url::DOMAIN_E, "|", eh_url::DOMAIN_LOFI, ")/s/([0-9a-f]{10})/(\\d+)-(\\d+)");
// const URL_PATTERN: &str = r#"([0-9a-f]{10})/(\d+)-(\d+)"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let url = r#"https://e-hentai.org/s/35142216f7/2062874-16"#;
        assert_eq!(GalleryPageUrl::parse(url).unwrap(), GalleryPageUrl {
            gid: 2062874,
            p_token: String::from("35142216f7"),
            page: 16,
        });
    }
}
