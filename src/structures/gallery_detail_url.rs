use regex::Regex;
use crate::{
    eh_url,
    EhResult,
    ParseError,
    Parser,
    const_concat,
    utils::parse_u64,
};

#[derive(Debug, PartialEq)]
pub struct GalleryDetailUrl {
    pub gid: u64,
    pub token: String,
}

impl Parser for GalleryDetailUrl {
    /// ```text
    /// https://e-hentai.org/g/2455981/acc72caed0/
    /// ```
    fn parse(doc: &str) -> EhResult<Self> {
        let regex = Regex::new(URL_STRICT_PATTERN).unwrap();

        let captures = regex.captures(doc).ok_or(ParseError::RegexMatchFailed)?;
        let gid = parse_u64(&captures[1])?;
        let token = String::from(&captures[2]);

        Ok(GalleryDetailUrl { gid, token })
    }
}

const URL_STRICT_PATTERN: &str = const_concat!("https?://(?:", eh_url::DOMAIN_EX, "|", eh_url::DOMAIN_E, "|", eh_url::DOMAIN_LOFI, ")/(?:g|mpv)/(\\d+)/([0-9a-f]{10})");
// const URL_PATTERN: &str = "(\\d+)/([0-9a-f]{10})(?:[^0-9a-f]|$)";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let url = "https://e-hentai.org/g/2455981/acc72caed0/";
        assert_eq!(GalleryDetailUrl::parse(url).is_ok(), true);
    }
}
