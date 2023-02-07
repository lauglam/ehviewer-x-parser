use visdom::Vis;

#[derive(Debug, PartialEq)]
pub struct Forums {
    /// Links to user profile page.
    pub user_link: String,
}

impl Forums {
    /// ```html
    /// <div id="userlinks"><p class="home"><b>Logged in as:  <a href="https://forums.e-hentai.org/index.php?showuser=xxxxx">
    ///                                                            ^
    ///                                                            This is we looking for.
    /// ```
    pub fn parse(doc: &str) -> Result<Forums, String> {
        if let Ok(root) = Vis::load(doc) {
            let user_link = root.find("#userlinks a");

            if let Some(href) = user_link.attr("href") {
                let user_link = href.to_string();

                return Ok(Forums {
                    user_link,
                });
            }
        }

        Err(String::from("parses forums fail."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::read_test_file;

    #[test]
    fn forums_parse_test() {
        let doc = read_test_file("forum.html");
        assert_eq!(Forums::parse(&doc).unwrap().user_link.is_empty(), false);
    }
}
