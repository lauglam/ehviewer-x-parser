use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::eh_config;

pub struct Category {
    pub color: u32,
    pub string: String,
    pub value: u32,
}

impl From<&String> for Category {
    fn from(value: &String) -> Self {
        Category::from(value.as_str())
    }
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        let strings = CATEGORY_STRINGS.lock().unwrap();

        let ids_opt = strings.iter()
            .position(|str_vec| str_vec.contains(&value));

        if let Some(idx) = ids_opt {
            Category::from(CATEGORY_VALUES[idx])
        } else {
            Category::from(CATEGORY_VALUES[10])
        }
    }
}

impl From<u32> for Category {
    fn from(value: u32) -> Self {
        let strings = CATEGORY_STRINGS.lock().unwrap();
        match value {
            eh_config::MISC => Category {
                string: String::from(strings[0][0]),
                color: BG_COLOR_MISC,
                value,
            },
            eh_config::DOUJINSHI => Category {
                string: String::from(strings[1][0]),
                color: BG_COLOR_DOUJINSHI,
                value,
            },
            eh_config::MANGA => Category {
                string: String::from(strings[2][0]),
                color: BG_COLOR_MANGA,
                value,
            },
            eh_config::ARTIST_CG => Category {
                string: String::from(strings[3][0]),
                color: BG_COLOR_ARTIST_CG,
                value,
            },
            eh_config::GAME_CG => Category {
                string: String::from(strings[4][0]),
                color: BG_COLOR_GAME_CG,
                value,
            },
            eh_config::IMAGE_SET => Category {
                string: String::from(strings[5][0]),
                color: BG_COLOR_IMAGE_SET,
                value,
            },
            eh_config::COSPLAY => Category {
                string: String::from(strings[6][0]),
                color: BG_COLOR_COSPLAY,
                value,
            },
            eh_config::ASIAN_PORN => Category {
                string: String::from(strings[7][0]),
                color: BG_COLOR_ASIAN_PORN,
                value,
            },
            eh_config::NON_H => Category {
                string: String::from(strings[8][0]),
                color: BG_COLOR_NON_H,
                value,
            },
            eh_config::WESTERN => Category {
                string: String::from(strings[9][0]),
                color: BG_COLOR_WESTERN,
                value,
            },
            _ => Category {
                string: String::from(strings[10][0]),
                color: BG_COLOR_UNKNOWN,
                value: UNKNOWN,
            },
        }
    }
}

// Use it for homepage
const NONE: i8 = -1;
const UNKNOWN: u32 = 0x400;

const ALL_CATEGORY: u32 = UNKNOWN - 1;

// DOUJINSHI|MANGA|ARTIST_CG|GAME_CG|WESTERN|NON_H|IMAGE_SET|COSPLAY|ASIAN_PORN|MISC;

const BG_COLOR_DOUJINSHI: u32 = 0xfff44336;
const BG_COLOR_MANGA: u32 = 0xffff9800;
const BG_COLOR_ARTIST_CG: u32 = 0xfffbc02d;
const BG_COLOR_GAME_CG: u32 = 0xff4caf50;
const BG_COLOR_WESTERN: u32 = 0xff8bc34a;
const BG_COLOR_NON_H: u32 = 0xff2196f3;
const BG_COLOR_IMAGE_SET: u32 = 0xff3f51b5;
const BG_COLOR_COSPLAY: u32 = 0xff9c27b0;
const BG_COLOR_ASIAN_PORN: u32 = 0xff9575cd;
const BG_COLOR_MISC: u32 = 0xfff06292;
const BG_COLOR_UNKNOWN: u32 = 0x00000000;

// Remove [XXX], (XXX), {XXX}, ~XXX~ stuff
const PATTERN_TITLE_PREFIX: &str = r#"^(?:(?:\([^\)]*\))|(?:\[[^\]]*\])|(?:\{[^\}]*\})|(?:~[^~]*~)|\s+)*"#;
// Remove [XXX], (XXX), {XXX}, ~XXX~ stuff and something like ch. 1-23
const PATTERN_TITLE_SUFFIX: &str = r#"(?:\s+ch.[\s\d-]+)?(?:(?:\([^\)]*\))|(?:\[[^\]]*\])|(?:\{[^\}]*\})|(?:~[^~]*~)|\s+)*$"#;

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
    UNKNOWN
];

static CATEGORY_STRINGS: Lazy<Mutex<[Vec<&str>; 11]>> = Lazy::new(|| {
    Mutex::new(
        [
            vec!["misc"],
            vec!["doujinshi"],
            vec!["manga"],
            vec!["artistcg", "Artist CG Sets", "Artist CG"],
            vec!["gamecg", "Game CG Sets", "Game CG"],
            vec!["imageset", "Image Sets", "Image Set"],
            vec!["cosplay"],
            vec!["asianporn", "Asian Porn"],
            vec!["non-h"],
            vec!["western"],
            vec!["unknown"],
        ]
    )
});
