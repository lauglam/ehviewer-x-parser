use visdom::Vis;
use crate::eh_url;

#[derive(Debug, PartialEq)]
pub struct Profile {
    pub display_name: String,
    pub avatar: String,
}

impl Profile {
    pub fn parse(document: &str) -> Result<Profile, String> {
        if let Ok(root) = Vis::load(document) {
            let display_name = root.find("#profilename > font");
            let display_name = display_name.text();

            let avatar = root.find(r#".ipbtable img"#);
            if let Some(avatar) = avatar.attr("src") {
                let mut avatar = avatar.to_string();
                if !avatar.starts_with("http") {
                    avatar = format!("{}{}", eh_url::URL_FORUMS, avatar);
                }

                return Ok(Profile {
                    display_name,
                    avatar,
                });
            }
        }

        Err(String::from("parses profile fail."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::read_test_file;
    use crate::eh_url;

    #[test]
    fn forums_parse_test() {
        let document = read_test_file("profile.html");

        assert_eq!(Profile::parse(&document).unwrap(), Profile {
            display_name: String::from(r#"xxxx"#),
            avatar: format!("{}{}", eh_url::URL_FORUMS, "style_images/ambience/warn0.gif"),
        });
    }
}
