use regex::Regex;
use crate::{EhResult, ParseError, Parser};

#[derive(Debug, PartialEq)]
pub struct SignIn {
    pub username: String,
}

impl Parser for SignIn {
    fn parse(doc: &str) -> EhResult<Self> {
        let regex = Regex::new(NAME_PATTERN).unwrap();
        if let Some(cap) = regex.captures(doc) {
            let username = String::from(&cap[1]);

            Ok(SignIn { username })
        } else {
            let regex = Regex::new(ERROR_PATTERN).unwrap();
            if let Some(cap) = regex.captures(doc) {
                let error = String::from(
                    if let Some(m) = cap.get(1) {
                        m.as_str()
                    } else {
                        &cap[2]
                    }
                );

                Err(ParseError::FromServer(error))
            } else {
                Err(ParseError::RegexMatchFailed)
            }
        }
    }
}

const NAME_PATTERN: &str = "<p>You are now logged in as: (.+?)<";
const ERROR_PATTERN: &str = r#"(?:<h4>The error returned was:</h4>\s*<p>(.+?)</p>)|(?:<span class="postcolor">(.+?)</span>)"#;

#[cfg(test)]
mod tests {
    use crate::utils::test::read_test_file;
    use super::*;

    #[test]
    fn parse_test() {
        let doc = read_test_file("sign_in_error.html");
        assert_eq!(SignIn::parse(&doc).is_err(), true);

        let doc = read_test_file("sign_in_success.html");
        assert_eq!(SignIn::parse(&doc).is_ok(), true);
    }
}
