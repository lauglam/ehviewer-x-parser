use visdom::Vis;
use crate::{eh_url, EhResult, ParseError, Parser};

#[derive(Debug, PartialEq)]
pub struct Profile {
    pub display_name: String,
    pub avatar: String,
}

impl Parser for Profile {
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;
        let display_name = root.find("#profilename > font");
        let display_name = display_name.text();

        let avatar = root.find(r#".ipbtable img"#);
        let avatar = avatar.attr("src").ok_or(ParseError::AttributeNotFound("src"))?;
        let mut avatar = avatar.to_string();
        if !avatar.starts_with("http") {
            avatar = format!("{}{}", eh_url::URL_FORUMS, avatar);
        }

        Ok(Profile {
            display_name,
            avatar,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::read_test_file;
    use crate::eh_url;

    #[test]
    fn forums_parse_test() {
        let doc = read_test_file("profile.html");

        assert_eq!(Profile::parse(&doc).unwrap(), Profile {
            display_name: String::from(r#"xxxx"#),
            avatar: format!("{}{}", eh_url::URL_FORUMS, "style_images/ambience/warn0.gif"),
        });
    }
}
