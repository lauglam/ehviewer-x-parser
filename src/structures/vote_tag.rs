use serde::{Deserialize, Serialize};
use crate::{EhResult, Parser};


#[derive(Debug, Serialize, Deserialize)]
pub struct VoteTag {
    pub error: String,
}

impl Parser for VoteTag {
    /// ```json
    /// {
    ///     "error": "The tag \"neko\" is not allowed. Use character:neko or artist:neko"
    /// }
    /// ```
    fn parse(doc: &str) -> EhResult<Self> {
        Ok(serde_json::from_str(doc)?)
    }
}
