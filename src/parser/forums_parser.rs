use visdom::Vis;

/// ```html
/// <div id="userlinks"><p class="home"><b>Logged in as:  <a href="https://forums.e-hentai.org/index.php?showuser=xxxxx">
///                                                            ^
///                                                            This is we looking for.
/// ```
pub fn parse(document: &str) -> Result<String, String> {
    let root = Vis::load(document).unwrap();
    let user_link = root.find("#userlinks a");

    match user_link.attr("href") {
        Some(href) => Ok(href.to_string()),
        None => Err(String::from("Parses forums fail."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::read_test_file;

    #[test]
    fn forums_parse_test() {
        let document = read_test_file("forum_parser");
        let _ = parse(&document).unwrap();
    }
}
