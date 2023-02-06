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
        if let Ok(rate_gallery) = serde_json::from_str::<RateGallery>(json) {
            Ok(rate_gallery)
        } else {
            Err(String::from("Parses rate gallery fail."))
        }
    }
}
