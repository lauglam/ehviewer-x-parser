use serde::{Deserialize, Serialize};
use crate::{EhResult, Parser};

#[derive(Debug, Serialize, Deserialize)]
pub struct RateGallery {
    #[serde(alias = "rating_avg")]
    pub rating: f32,
    #[serde(alias = "rating_cnt")]
    pub rating_count: i32,
}

impl Parser for RateGallery{
    fn parse(doc: &str) -> EhResult<Self> {
        Ok(serde_json::from_str(doc)?)
    }
}
