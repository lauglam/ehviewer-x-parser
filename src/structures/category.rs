use crate::eh_config;

trait IgnoreCase {
    fn contains_ignore_case(&self, x: &str) -> bool;
}

impl IgnoreCase for [&str] {
    fn contains_ignore_case(&self, x: &str) -> bool {
        self.iter().any(|s| s.to_lowercase() == x.to_lowercase())
    }
}

#[derive(Debug, PartialEq)]
pub struct Category {
    pub color: u32,
    pub string: String,
    pub value: u32,
}

impl From<&String> for Category {
    fn from(value: &String) -> Self {
        for (idx, strings) in CATEGORY_STRINGS.iter().enumerate() {
            if !strings.contains_ignore_case(&value.as_str()) {
                continue;
            }

            return Category {
                string: String::from(value),
                color: CATEGORY_COLORS[idx],
                value: CATEGORY_VALUES[idx],
            };
        }

        // unknown.
        Category {
            string: String::from(CATEGORY_STRINGS[10][0]),
            color: CATEGORY_COLORS[10],
            value: CATEGORY_VALUES[10],
        }
    }
}

impl From<u32> for Category {
    fn from(value: u32) -> Self {
        let idx_opt = CATEGORY_VALUES.iter().position(|v| *v == value);
        if let Some(idx) = idx_opt {
            Category {
                color: CATEGORY_COLORS[idx],
                string: String::from(CATEGORY_STRINGS[idx][0]),
                value,
            }
        } else {
            // unknown.
            Category {
                string: String::from(CATEGORY_STRINGS[10][0]),
                color: CATEGORY_COLORS[10],
                value: CATEGORY_VALUES[10],
            }
        }
    }
}

// Use it for homepage
const NONE: i8 = -1;

const ALL_CATEGORY: u32 = VALUE_UNKNOWN - 1;


// Remove [XXX], (XXX), {XXX}, ~XXX~ stuff
const PATTERN_TITLE_PREFIX: &str = r#"^(?:(?:\([^\)]*\))|(?:\[[^\]]*\])|(?:\{[^\}]*\})|(?:~[^~]*~)|\s+)*"#;
// Remove [XXX], (XXX), {XXX}, ~XXX~ stuff and something like ch. 1-23
const PATTERN_TITLE_SUFFIX: &str = r#"(?:\s+ch.[\s\d-]+)?(?:(?:\([^\)]*\))|(?:\[[^\]]*\])|(?:\{[^\}]*\})|(?:~[^~]*~)|\s+)*$"#;

// DOUJINSHI|MANGA|ARTIST_CG|GAME_CG|WESTERN|NON_H|IMAGE_SET|COSPLAY|ASIAN_PORN|MISC;

const CATEGORY_COLORS: [u32; 11] = [
    BG_COLOR_MISC,
    BG_COLOR_DOUJINSHI,
    BG_COLOR_MANGA,
    BG_COLOR_ARTIST_CG,
    BG_COLOR_GAME_CG,
    BG_COLOR_IMAGE_SET,
    BG_COLOR_COSPLAY,
    BG_COLOR_ASIAN_PORN,
    BG_COLOR_NON_H,
    BG_COLOR_WESTERN,
    BG_COLOR_UNKNOWN
];

const CATEGORY_VALUES: [u32; 11] = [
    eh_config::MISC,
    eh_config::DOUJINSHI,
    eh_config::MANGA,
    eh_config::ARTIST_CG,
    eh_config::GAME_CG,
    eh_config::IMAGE_SET,
    eh_config::COSPLAY,
    eh_config::ASIAN_PORN,
    eh_config::NON_H,
    eh_config::WESTERN,
    VALUE_UNKNOWN
];

const VALUE_UNKNOWN: u32 = 0x400;

const CATEGORY_STRINGS: [[&str; 3]; 11] = [
    ["misc", EMPTY_STRING, EMPTY_STRING],
    ["doujinshi", EMPTY_STRING, EMPTY_STRING],
    ["manga", EMPTY_STRING, EMPTY_STRING],
    ["artistcg", "Artist CG Sets", "Artist CG"],
    ["gamecg", "Game CG Sets", "Game CG"],
    ["imageset", "Image Sets", "Image Set"],
    ["cosplay", EMPTY_STRING, EMPTY_STRING],
    ["asianporn", "Asian Porn", EMPTY_STRING],
    ["non-h", EMPTY_STRING, EMPTY_STRING],
    ["western", EMPTY_STRING, EMPTY_STRING],
    ["unknown", EMPTY_STRING, EMPTY_STRING],
];

const BG_COLOR_MISC: u32 = 0xfff06292;
const BG_COLOR_DOUJINSHI: u32 = 0xfff44336;
const BG_COLOR_MANGA: u32 = 0xffff9800;
const BG_COLOR_ARTIST_CG: u32 = 0xfffbc02d;
const BG_COLOR_GAME_CG: u32 = 0xff4caf50;
const BG_COLOR_IMAGE_SET: u32 = 0xff3f51b5;
const BG_COLOR_COSPLAY: u32 = 0xff9c27b0;
const BG_COLOR_ASIAN_PORN: u32 = 0xff9575cd;
const BG_COLOR_NON_H: u32 = 0xff2196f3;
const BG_COLOR_WESTERN: u32 = 0xff8bc34a;
const BG_COLOR_UNKNOWN: u32 = 0x00000000;

const EMPTY_STRING: &str = "";
