use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteTag {
    pub error: String,
}

/// ```json
/// {
///     "error": "The tag \"neko\" is not allowed. Use character:neko or artist:neko"
/// }
/// ```
impl VoteTag {
    pub fn parse(json: &str) -> Result<VoteTag, String> {
        serde_json::from_str::<VoteTag>(json).map_err(|_| String::from("parses vote tag fail."))
    }
}
