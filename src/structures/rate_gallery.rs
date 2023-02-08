use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RateGallery {
    #[serde(alias = "rating_avg")]
    pub rating: f32,
    #[serde(alias = "rating_cnt")]
    pub rating_count: i32,
}

impl RateGallery {
    pub fn parse(json: &str) -> Result<RateGallery, String> {
        serde_json::from_str::<RateGallery>(json).map_err(|_| String::from("parses rate gallery fail."))
    }
}