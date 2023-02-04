use regex::Regex;
use crate::const_concat;
use crate::eh_url;
use crate::utils::parse_i64;

pub struct GalleryDetailUrl {
    pub gid: i64,
    pub token: String,
}

impl GalleryDetailUrl {
    pub fn parse(url: &str, strict: bool) -> Result<GalleryDetailUrl, String> {
        let regex = Regex::new(if strict { URL_STRICT_PATTERN } else { URL_PATTERN }).unwrap();
        if regex.is_match(url) {
            let cap = regex.captures(url).unwrap();
            let gid = parse_i64(&cap[1], -1 as i64);
            let token = String::from(&cap[2]);

            if gid > 0 {
                return Ok(GalleryDetailUrl {
                    gid,
                    token,
                });
            }
        }

        Err(String::from("Parses gallery detail url fail."))
    }
}

const URL_STRICT_PATTERN: &str = const_concat!("https?://(?:", eh_url::DOMAIN_EX, "|", eh_url::DOMAIN_E, "|", eh_url::DOMAIN_LO_FI, ")/(?:g|mpv)/(\\d+)/([0-9a-f]{10})");
const URL_PATTERN: &str = "(\\d+)/([0-9a-f]{10})(?:[^0-9a-f]|$)";

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
