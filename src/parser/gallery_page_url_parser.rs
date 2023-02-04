use regex::Regex;
use crate::const_concat;
use crate::eh_url;
use crate::utils::{parse_i64, parse_usize};

pub struct GalleryPageUrl {
    pub gid: i64,
    pub p_token: String,
    pub page: usize,
}

impl GalleryPageUrl {
    pub fn parse(url: &str, strict: bool) -> Result<GalleryPageUrl, String> {
        const URL_STRICT_PATTERN: &str = const_concat!("https?://(?:", eh_url::DOMAIN_EX, "|", eh_url::DOMAIN_E, "|", eh_url::DOMAIN_LO_FI, ")/s/([0-9a-f]{10})/(\\d+)-(\\d+)");
        const URL_PATTERN: &str = "([0-9a-f]{10})/(\\d+)-(\\d+)";

        let regex = Regex::new(if strict { URL_STRICT_PATTERN } else { URL_PATTERN }).unwrap();
        if regex.is_match(url) {
            let mut ms = regex.find_iter(url);
            let p_token = String::from(ms.nth(1).unwrap().as_str());
            let gid = parse_i64(ms.nth(2).unwrap().as_str(), -1 as i64);
            let page = parse_usize(ms.nth(2).unwrap().as_str(), 0) - 1;

            if gid > 0 && page > 0 {
                return Ok(GalleryPageUrl {
                    gid,
                    p_token,
                    page,
                });
            }
        }

        Err(String::from("Parses gallery page url fail."))
    }
}
