use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteComment {
    #[serde(alias = r#"comment_id"#)]
    pub id: u64,
    #[serde(alias = r#"comment_score"#)]
    pub score: i32,
    #[serde(alias = r#"comment_vote"#)]
    pub vote: u32,
    // TODO need expect_vote?
    // pub expect_vote: u32,
}

impl VoteComment {
    /// ```json
    /// {
    ///     "comment_id": 1253922,
    ///     "comment_score": -19,
    ///     "comment_vote": 0
    /// }
    /// ```
    pub fn parse(json: &str) -> Result<VoteComment, String> {
        serde_json::from_str(json).map_err(|_| String::from("parses vote comment fail."))
    }
}