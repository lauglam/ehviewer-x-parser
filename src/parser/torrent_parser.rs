use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Torrent {
    pub filename: String,
    pub download_url: String,
}

impl Torrent {
    pub fn parse(element: &str) -> Result<Torrent, String> {
        const PATTERN_TORRENT: &str = r#"<td colspan="5"> &nbsp; <a href=".*" onclick="document.location='([^"]+)'[^<]+>([^<]+)</a></td>"#;

        let regex = Regex::new(PATTERN_TORRENT).unwrap();
        if let Some(cap) = regex.captures(element) {
            let download_url = String::from(&cap[1]);
            let filename = String::from(&cap[2]);

            Ok(Torrent {
                filename,
                download_url,
            })
        } else {
            Err(String::from("Parses torrent fail."))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let element = r#"
            <tr>
                <td colspan="5"> &nbsp; <a href="https://ehtracker.org/get/2257278/9a16691657fb9ec9ad124298af12eaaf86fa614c.torrent" onclick="document.location='https://ehtracker.org/get/xxxx/xxxx.torrent?p=xxxx'; return false">xxxx.zip</a></td>
            </tr>
            "#;

        assert_eq!(Torrent::parse(element).unwrap(),Torrent{
            filename:String::from("xxxx.zip"),
            download_url:String::from("https://ehtracker.org/get/xxxx/xxxx.torrent?p=xxxx")
        });
    }
}
