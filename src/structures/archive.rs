use regex::Regex;
use crate::{EhResult, Parser, REGEX_MATCH_FAILED, unescape::unescape};

#[derive(Debug, PartialEq)]
pub struct Archive {
    pub or: String,
    pub items: Vec<Item>,
}

#[derive(Debug, PartialEq)]
pub struct Item {
    pub res: String,
    pub name: String,
}

impl Parser for Archive {
    fn parse(doc: &str) -> EhResult<Self> {
        let regex = Regex::new(PATTERN_FORM).unwrap();
        let captures = regex.captures(doc).ok_or(REGEX_MATCH_FAILED)?;
        let or = String::from(&captures[1]);

        let regex = Regex::new(PATTERN_ARCHIVE).unwrap();
        let mut items = Vec::new();

        for cap in regex.captures_iter(doc) {
            let res = String::from(unescape(&cap[1]));
            let name = String::from(unescape(&cap[2]));

            items.push(Item {
                res,
                name,
            });
        }

        Ok(Archive { or, items })
    }
}

const PATTERN_FORM: &str = r#"<form id="hathdl_form" action="[^"]*?or=([^="]*?)" method="post">"#;
const PATTERN_ARCHIVE: &str = r#"<a href="[^"]*" onclick="return do_hathdl\('([0-9]+|org)'\)">([^<]+)</a>"#;

#[cfg(test)]
mod tests {
    use crate::test_helper::read_test_file;
    use super::*;

    #[test]
    fn parse_test() {
        let doc = read_test_file("archive.html");
        assert_eq!(Archive::parse(&doc).is_ok(), true);
    }
}
