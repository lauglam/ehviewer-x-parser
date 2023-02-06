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
        if let Ok(vote_tag) = serde_json::from_str::<VoteTag>(json) {
            Ok(vote_tag)
        } else {
            Err(String::from("Parses vote tag fail."))
        }
    }
}
