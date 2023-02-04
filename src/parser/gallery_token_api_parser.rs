use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    gid: i64,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GalleryToken {
    #[serde(alias = r#"tokenlist"#)]
    token_vec: Option<Vec<Token>>,
    error: String,
}

/// ```json
/// {
///     "tokenlist": [
///         {
///             "gid": 618395,
///             "token": "0439fa3666"
///         }
///     ],
///     "error": "maomao is moe~"
/// }
/// ```
pub fn parse(json: &str) -> Result<String, String> {
    let gallery_token: GalleryToken = serde_json::from_str(json).unwrap();
    if let Some(token_vec) = gallery_token.token_vec {
        Ok(String::from(&token_vec[0].token))
    } else {
        Err(gallery_token.error)
    }
}
