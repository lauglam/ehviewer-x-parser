use regex::Regex;
use crate::{EhResult, Parser, REGEX_MATCH_FAILED};

#[derive(Debug, PartialEq)]
pub struct Torrent {
    pub filename: String,
    pub download_url: String,
}

impl Parser for Torrent {
    fn parse(doc: &str) -> EhResult<Self> {
        let regex = Regex::new(PATTERN_TORRENT).unwrap();
        let captures = regex.captures(doc).ok_or(REGEX_MATCH_FAILED)?;

        let download_url = String::from(&captures[1]);
        let filename = String::from(&captures[2]);

        Ok(Torrent {
            filename,
            download_url,
        })
    }
}

const PATTERN_TORRENT: &str = r#"<td colspan="5"> &nbsp; <a href=".*" onclick="document.location='([^"]+)'[^<]+>([^<]+)</a></td>"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let ele = r#"
            <tr>
                <td colspan="5"> &nbsp; <a href="https://ehtracker.org/get/2257278/9a16691657fb9ec9ad124298af12eaaf86fa614c.torrent" onclick="document.location='https://ehtracker.org/get/xxxx/xxxx.torrent?p=xxxx'; return false">xxxx.zip</a></td>
            </tr>
        "#;

        assert_eq!(Torrent::parse(ele).unwrap(), Torrent {
            filename: String::from(r#"xxxx.zip"#),
            download_url: String::from("https://ehtracker.org/get/xxxx/xxxx.torrent?p=xxxx"),
        });
    }
}
