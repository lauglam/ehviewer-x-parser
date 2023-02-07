use regex::Regex;
use crate::const_concat;
use crate::eh_url;
use crate::utils::{
    parse_u32,
    parse_u64,
};

#[derive(Debug, PartialEq)]
pub struct GalleryPageUrl {
    pub gid: u64,
    pub p_token: String,
    pub page: u32,
}

impl GalleryPageUrl {
    pub fn parse(url: &str, strict: bool) -> Result<GalleryPageUrl, String> {
        const URL_STRICT_PATTERN: &str = const_concat!("https?://(?:", eh_url::DOMAIN_EX, "|", eh_url::DOMAIN_E, "|", eh_url::DOMAIN_LOFI, ")/s/([0-9a-f]{10})/(\\d+)-(\\d+)");
        const URL_PATTERN: &str = r#"([0-9a-f]{10})/(\d+)-(\d+)"#;

        let regex = Regex::new(if strict { URL_STRICT_PATTERN } else { URL_PATTERN }).unwrap();
        if let Some(cap) = regex.captures(url) {
            let p_token = String::from(&cap[1]);
            let gid = parse_u64(&cap[2])?;
            let page = parse_u32(&cap[3])?;

            if gid > 0 && page > 0 {
                return Ok(GalleryPageUrl {
                    gid,
                    p_token,
                    page,
                });
            }
        }

        Err(String::from("parses gallery page url fail."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let url = r#"https://e-hentai.org/s/35142216f7/2062874-16"#;
        assert_eq!(GalleryPageUrl::parse(url, true).unwrap(), GalleryPageUrl {
            gid: 2062874,
            p_token: String::from("35142216f7"),
            page: 16,
        });
    }
}
