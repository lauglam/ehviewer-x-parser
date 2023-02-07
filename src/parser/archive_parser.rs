use regex::Regex;
use crate::utils::unescape;

pub struct Archive {
    pub or: String,
    pub items: Vec<Item>,
}

pub struct Item {
    pub res: String,
    pub name: String,
}

pub fn parse(document: &str) -> Option<Archive> {
    const PATTERN_FORM: &str = r#"<form id="hathdl_form" action="[^"]*?or=([^="]*?)" method="post">"#;
    const PATTERN_ARCHIVE: &str = r#"<a href="[^"]*" onclick="return do_hathdl\('([0-9]+|org)'\)">([^<]+)</a>"#;

    let form_reg = Regex::new(PATTERN_FORM).unwrap();
    let archive_reg = Regex::new(PATTERN_ARCHIVE).unwrap();

    match form_reg.find(document) {
        None => None,
        Some(m) => {
            let or = String::from(m.as_str());
            let mut items = Vec::new();

            for cap in archive_reg.captures_iter(document) {
                let res = unescape(&cap[1]).into_owned();
                let name = unescape(&cap[2]).into_owned();

                items.push(Item {
                    res,
                    name,
                });
            }

            Some(Archive {
                or,
                items,
            })
        }
    }
}
