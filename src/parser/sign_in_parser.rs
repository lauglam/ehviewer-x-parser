use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct SignIn {
    pub username_opt: Option<String>,
    pub error_opt: Option<String>,
}

impl SignIn {
    pub fn parse(document: &str) -> Result<SignIn, String> {
        const NAME_PATTERN: &str = "<p>You are now logged in as: (.+?)<";
        const ERROR_PATTERN: &str = r#"(?:<h4>The error returned was:</h4>\s*<p>(.+?)</p>)|(?:<span class="postcolor">(.+?)</span>)"#;

        let regex = Regex::new(NAME_PATTERN).unwrap();
        if let Some(cap) = regex.captures(document) {
            let username_opt = Some(String::from(&cap[1]));

            Ok(SignIn {
                username_opt,
                error_opt: None,
            })
        } else {
            let regex = Regex::new(ERROR_PATTERN).unwrap();
            if let Some(cap) = regex.captures(document) {
                let error_opt = Some(String::from(
                    if let Some(m) = cap.get(1) {
                        m.as_str()
                    } else {
                        &cap[2]
                    }
                ));

                Ok(SignIn {
                    username_opt: None,
                    error_opt,
                })
            } else {
                Err(String::from("Parses sign in fail."))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test::read_test_file;
    use super::*;

    #[test]
    fn parse_test() {
        let document = read_test_file("sign_in_parser_error");
        assert_eq!(SignIn::parse(&document).unwrap(), SignIn {
            username_opt: None,
            error_opt: Some(String::from("The captcha was not entered correctly. Please try again.")),
        });

        let document = read_test_file("sign_in_parser_success");
        assert_eq!(SignIn::parse(&document).unwrap().username_opt.is_some(), true);
    }
}
