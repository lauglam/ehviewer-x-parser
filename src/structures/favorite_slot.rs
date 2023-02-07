use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct FavoriteSlot {
    pub r: String,
    pub g: String,
    pub b: String,
    pub value: u32,
}

impl FavoriteSlot {
    pub fn parse(style: &str) -> Result<FavoriteSlot, String> {
        const FAVORITE_SLOT_RGB: [[&str; 3]; 10] = [
            ["0", "0", "0"],
            ["240", "0", "0"],
            ["240", "160", "0"],
            ["208", "208", "0"],
            ["0", "128", "0"],
            ["144", "240", "64"],
            ["64", "176", "240"],
            ["0", "0", "240"],
            ["80", "0", "128"],
            ["224", "128", "224"],
        ];

        const PATTERN_FAVORITE_SLOT: &str = r#"background-color:rgba\((\d+),(\d+),(\d+),"#;
        let regex = Regex::new(PATTERN_FAVORITE_SLOT).unwrap();

        if let Some(cap) = regex.captures(style) {
            let r = &cap[1];
            let g = &cap[2];
            let b = &cap[3];

            let mut slot = 0;
            for rgb in FAVORITE_SLOT_RGB {
                if r == rgb[0] && g == rgb[1] && b == rgb[2] {
                    return Ok(FavoriteSlot {
                        r: String::from(r),
                        g: String::from(g),
                        b: String::from(b),
                        value: slot,
                    });
                }

                slot += 1;
            }
        }

        Err(String::from("parses favorite slot fail."))
    }
}
