use visdom::Vis;
use crate::{EhResult, ParseError, Parser};

#[derive(Debug, PartialEq)]
pub struct Forums {
    /// Links to user profile page.
    pub user_link: String,
}

impl Parser for Forums {
    /// ```html
    /// <div id="userlinks"><p class="home"><b>Logged in as:  <a href="https://forums.e-hentai.org/index.php?showuser=xxxxx">
    ///                                                            ^
    ///                                                            This is we looking for.
    /// ```
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;
        let user_link = root.find("#userlinks a");

        let href = user_link.attr("href").ok_or(ParseError::AttributeNotFound("href"))?;
        let user_link = href.to_string();

        Ok(Forums { user_link })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::read_test_file;

    #[test]
    fn forums_parse_test() {
        let doc = read_test_file("forums.html");
        assert_eq!(Forums::parse(&doc).is_ok(), true);
    }
}
