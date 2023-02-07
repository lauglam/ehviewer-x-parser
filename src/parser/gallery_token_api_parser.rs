use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Token {
    gid: u64,
    token: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct GalleryToken {
    #[serde(alias = r#"tokenlist"#)]
    token_vec_opt: Option<Vec<Token>>,
    #[serde(alias = r#"error"#)]
    error_opt: Option<String>,
}

impl GalleryToken {
    /// ```json
    /// {
    ///     "tokenlist": [
    ///         {
    ///             "gid": 2062874,
    ///             "token": "03037d8698"
    ///         }
    ///     ]
    /// }
    /// ```
    /// Or
    /// ```json
    /// {
    ///     "error": "maomao is moe~"
    /// }
    /// ```
    pub fn parse(json: &str) -> Result<GalleryToken, String> {
        if let Ok(gallery_token) = serde_json::from_str::<GalleryToken>(json) {
            Ok(gallery_token)
        } else {
            Err(String::from("parses gallery token api fail."))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let json = r#"
            {
                "tokenlist": [
                    {
                        "gid": 2062874,
                        "token": "03037d8698"
                    }
                ]
            }
            "#;

        assert_eq!(GalleryToken::parse(json).unwrap(), GalleryToken {
            token_vec_opt: Some(vec![Token {
                gid: 2062874,
                token: String::from("03037d8698"),
            }]),
            error_opt: None,
        });

        let json = r#"
            {
                "error": "maomao is moe~"
            }
            "#;

        assert_eq!(GalleryToken::parse(json).unwrap(), GalleryToken {
            token_vec_opt: None,
            error_opt: Some(String::from(r#"maomao is moe~"#)),
        });
    }
}
