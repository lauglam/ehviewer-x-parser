use regex::Regex;
use crate::const_concat;
use crate::eh_url;
use crate::utils::parse_u64;

#[derive(Debug, PartialEq)]
pub struct GalleryDetailUrl {
    pub gid: u64,
    pub token: String,
}

impl GalleryDetailUrl {
    /// ```text
    /// https://e-hentai.org/g/2455981/acc72caed0/
    /// ```
    pub fn parse(href: &str, strict: bool) -> Result<GalleryDetailUrl, String> {
        const URL_STRICT_PATTERN: &str = const_concat!("https?://(?:", eh_url::DOMAIN_EX, "|", eh_url::DOMAIN_E, "|", eh_url::DOMAIN_LOFI, ")/(?:g|mpv)/(\\d+)/([0-9a-f]{10})");
        const URL_PATTERN: &str = "(\\d+)/([0-9a-f]{10})(?:[^0-9a-f]|$)";

        let regex = Regex::new(if strict { URL_STRICT_PATTERN } else { URL_PATTERN }).unwrap();
        if regex.is_match(href) {
            let cap = regex.captures(href).unwrap();
            let gid = parse_u64(&cap[1])?;
            let token = String::from(&cap[2]);

            if gid > 0 {
                return Ok(GalleryDetailUrl {
                    gid,
                    token,
                });
            }
        }

        Err(String::from("parses gallery detail url fail."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let url = "https://e-hentai.org/g/2455981/acc72caed0/";
        let result = GalleryDetailUrl::parse(url, true);
        if result.is_err() {
            panic!()
        }
    }
}