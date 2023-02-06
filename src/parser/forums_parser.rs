use visdom::Vis;

#[derive(Debug, PartialEq)]
pub struct Forum {
    /// Links to user profile page.
    pub user_link: String,
}

impl Forum {
    /// ```html
    /// <div id="userlinks"><p class="home"><b>Logged in as:  <a href="https://forums.e-hentai.org/index.php?showuser=xxxxx">
    ///                                                            ^
    ///                                                            This is we looking for.
    /// ```
    pub fn parse(document: &str) -> Result<Forum, String> {
        if let Ok(root) = Vis::load(document) {
            let user_link = root.find("#userlinks a");

            if let Some(href) = user_link.attr("href") {
                let user_link = href.to_string();

                return Ok(Forum {
                    user_link,
                });
            }
        }

        Err(String::from("Parses forums fail."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::read_test_file;

    #[test]
    fn forums_parse_test() {
        let document = read_test_file("forum_parser");

        assert_eq!(Forum::parse(&document).unwrap().user_link,
                   r#"https://forums.e-hentai.org/index.php?showuser=xxxxx"#);
    }
}
