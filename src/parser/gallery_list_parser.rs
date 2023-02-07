#![allow(dead_code)]

use core::f32;
use regex::Regex;
use visdom::types::Elements;
use visdom::Vis;
use crate::parser::category_parser::Category;
use crate::utils::{
    parse_i32,
    parse_u32,
};
use crate::parser::gallery_detail_url_parser::GalleryDetailUrl;

#[derive(Debug, PartialEq)]
pub struct GalleryInfo {
    pub gid: u64,
    pub token: String,
    pub title: String,
    pub title_jpn_opt: Option<String>,
    pub thumb: String,
    pub thumb_width: u32,
    pub thumb_height: u32,
    pub category: u32,
    pub posted: String,
    pub uploader_opt: Option<String>,
    pub rating: f32,
    pub simple_tag_vec_opt: Option<Vec<String>>,
    pub pages: u32,
    pub simple_language_opt: Option<String>,
    pub favorite_name_opt: Option<String>,
    pub favorite_slot: i32,
}

impl GalleryInfo {
    pub fn parse(element: &str, inline: &Inline) -> Result<GalleryInfo, String> {
        if let Ok(root) = Vis::load(element) {
            let gl_name = root.find(r#".glname"#);

            // Gid and Token.
            let gid;
            let token;
            {
                let url = match inline {
                    Inline::MinimalOrMinimalPlus | Inline::Compact | Inline::Thumbnail => {
                        gl_name.find(r#"a"#)
                    }
                    Inline::Extended => {
                        gl_name.parent("a")
                    }
                };

                let detail_url = GalleryDetailUrl::parse(&url.text(), true)?;
                gid = detail_url.gid;
                token = detail_url.token;
            }

            // Tag.
            let mut simple_tag_vec_opt: Option<Vec<String>> = None;
            match inline {
                Inline::Compact | Inline::Extended => {
                    let gts = gl_name.find(".gt");
                    let mut simple_tag_vec = Vec::new();
                    for gt in gts {
                        let title_attr = gt.get_attribute("title").unwrap();
                        simple_tag_vec.push(title_attr.to_string())
                    }

                    simple_tag_vec_opt.replace(simple_tag_vec);
                }
                _ => {}
            }

            // Category.
            let category;
            {
                let cs_or_cn;
                match inline {
                    Inline::MinimalOrMinimalPlus | Inline::Thumbnail => {
                        cs_or_cn = root.find(".gl1m > .cs");
                    }
                    Inline::Compact | Inline::Extended => {
                        cs_or_cn = root.find(".cn");
                    }
                }
                category = Category::from(&cs_or_cn.text()).value;
            }

            // Thumb and Pages and Rating.
            let pages;
            let thumb;
            let thumb_height;
            let thumb_width;
            let rating;
            {
                let img;
                let ir;
                let pages_str;
                match inline {
                    Inline::MinimalOrMinimalPlus | Inline::Compact => {
                        // Thumb.
                        let gl_thumb = root.find(r#".glthumb"#);
                        img = gl_thumb.find("img");

                        // Pages and Rating.
                        ir = gl_thumb.find(".ir");
                        pages_str = ir.text();
                    }
                    Inline::Extended => {
                        // Thumb.
                        img = root.find("img");

                        // Pages and Rating.
                        ir = root.find(".ir");
                        pages_str = ir.next("").next("").text();
                    }
                    Inline::Thumbnail => {
                        // Thumb.
                        img = root.find("img");

                        // Pages and Rating.
                        ir = root.find(".ir");
                        pages_str = ir.next("").text();
                    }
                }

                // Thumb.
                let style = img.attr("style").unwrap().to_string();

                const PATTERN_THUMB_SIZE: &str = r#"height:(\d+)px;width:(\d+)px"#;
                let regex = Regex::new(PATTERN_THUMB_SIZE).unwrap();
                let captures = regex.captures(&style).unwrap();

                thumb_height = parse_u32(&captures[1])?;
                thumb_width = parse_u32(&captures[2])?;

                // TODO setting
                let src = img.attr("src").unwrap().to_string();
                thumb = src;

                // Pages
                const PATTERN_PAGES: &str = r#"(\d+) page"#;
                let regex = Regex::new(PATTERN_PAGES).unwrap();
                let captures = regex.captures(&pages_str).unwrap();

                pages = parse_u32(&captures[1])?;

                // Rating.
                let style = ir.attr("style").unwrap();
                rating = parse_rating(&style.to_string())?;
            }

            // Posted and FavoriteSlot.
            let mut favorite_slot = -2;
            let posted;
            {
                let posted_ele = root.find(&format!("#posted_{}", gid));
                if let Some(slot) = posted_ele.attr("style") {
                    favorite_slot = parse_favorite_slot(&slot.to_string());
                }
                posted = posted_ele.text();
            }

            // Uploader.
            let mut uploader_opt = None;
            match inline {
                Inline::MinimalOrMinimalPlus | Inline::Compact | Inline::Extended => {
                    let prefix = r#"https://e-hentai.org/uploader/"#;
                    let uploader = root.find(&format!("[href^={}]", prefix));
                    uploader_opt.replace(uploader.text());
                }
                _ => {}
            }

            // Title.
            let title = root.find(r#".glink"#).text();

            // SimpleLanguage.
            let mut simple_language_opt = None;
            if let Some(ref simple_tag_vec) = simple_tag_vec_opt {
                for tag in simple_tag_vec {
                    let idx_opt = S_LANG_TAGS.iter().position(|&t| t == tag);
                    if let Some(idx) = idx_opt {
                        simple_language_opt.replace(String::from(S_LANGS[idx]));
                    }
                }
            } else {
                let idx_opt = S_LANG_PATTERNS.iter().position(|pattern| {
                    let regex = Regex::new(pattern).unwrap();
                    regex.is_match(&title)
                });

                if let Some(idx) = idx_opt {
                    simple_language_opt.replace(String::from(S_LANGS[idx]));
                }
            }

            Ok(GalleryInfo {
                gid,
                token,
                title,
                pages,
                thumb,
                thumb_width,
                thumb_height,
                rating,
                posted,
                category,
                uploader_opt,
                favorite_slot,
                simple_tag_vec_opt,
                simple_language_opt,
                title_jpn_opt: None,
                favorite_name_opt: None,
            })
        } else {
            Err(String::from("parses gallery info fail."))
        }
    }

    pub fn available_title(&self) -> &str {
        if let Some(ref title_jpn) = self.title_jpn_opt {
            title_jpn
        } else {
            self.title.as_ref()
        }
    }
}

impl ToString for GalleryInfo {
    fn to_string(&self) -> String {
        if let Some(ref title_jpn) = self.title_jpn_opt {
            format!("{} {}", self.gid, title_jpn)
        } else {
            format!("{} {}", self.gid, self.title)
        }
    }
}

// Regex for GalleryInfo.

const S_LANG_JA: &str = "JA";
const S_LANG_EN: &str = "EN";
const S_LANG_ZH: &str = "ZH";
const S_LANG_NL: &str = "NL";
const S_LANG_FR: &str = "FR";
const S_LANG_DE: &str = "DE";
const S_LANG_HU: &str = "HU";
const S_LANG_IT: &str = "IT";
const S_LANG_KO: &str = "KO";
const S_LANG_PL: &str = "PL";
const S_LANG_PT: &str = "PT";
const S_LANG_RU: &str = "RU";
const S_LANG_ES: &str = "ES";
const S_LANG_TH: &str = "TH";
const S_LANG_VI: &str = "VI";

const S_LANGS: [&str; 14] = [
    "S_LANG_EN",
    "S_LANG_ZH",
    "S_LANG_ES",
    "S_LANG_KO",
    "S_LANG_RU",
    "S_LANG_FR",
    "S_LANG_PT",
    "S_LANG_TH",
    "S_LANG_DE",
    "S_LANG_IT",
    "S_LANG_VI",
    "S_LANG_PL",
    "S_LANG_HU",
    "S_LANG_NL",
];

const S_LANG_PATTERNS: [&str; 14] = [
    r#"[(\[]eng(?:lish)?[)\]]|英訳"#,
    // r#[(（\[]ch(?:inese)?[)）\]]|[汉漢]化|中[国國][语語]|中文|中国翻訳#,
    r#"[(\uFF08\[]ch(?:inese)?[)\uFF09\]]|[汉漢]化|中[国國][语語]|中文|中国翻訳"#,
    r#"[(\[]spanish[)\]]|[(\[]Español[)\]]|スペイン翻訳"#,
    r#"[(\[]korean?[)\]]|韓国翻訳"#,
    r#"[(\[]rus(?:sian)?[)\]]|ロシア翻訳"#,
    r#"[(\[]fr(?:ench)?[)\]]|フランス翻訳"#,
    r#"[(\[]portuguese|ポルトガル翻訳"#,
    r#"[(\[]thai(?: ภาษาไทย)?[)\]]|แปลไทย|タイ翻訳"#,
    r#"[(\[]german[)\]]|ドイツ翻訳"#,
    r#"[(\[]italiano?[)\]]|イタリア翻訳"#,
    r#"[(\[]vietnamese(?: Tiếng Việt)?[)\]]|ベトナム翻訳"#,
    r#"[(\[]polish[)\]]|ポーランド翻訳"#,
    r#"[(\[]hun(?:garian)?[)\]]|ハンガリー翻訳"#,
    r#"[(\[]dutch[)\]]|オランダ翻訳"#,
];

const S_LANG_TAGS: [&str; 14] = [
    "language:english",
    "language:chinese",
    "language:spanish",
    "language:korean",
    "language:russian",
    "language:french",
    "language:portuguese",
    "language:thai",
    "language:german",
    "language:italian",
    "language:vietnamese",
    "language:polish",
    "language:hungarian",
    "language:dutch",
];

pub enum Inline {
    MinimalOrMinimalPlus,
    Compact,
    Extended,
    Thumbnail,
}

impl Inline {
    pub fn parse(element: &Elements) -> Result<Inline, String> {
        if element.has_class(r#"gltm"#) {
            // Minimal or Minimal+.
            Ok(Inline::MinimalOrMinimalPlus)
        } else if element.has_class(r#"gltc"#) {
            // Compact.
            Ok(Inline::Compact)
        } else if element.has_class(r#"glte"#) {
            // Extended.
            Ok(Inline::Extended)
        } else if element.has_class("gld") {
            // Thumbnail.
            Ok(Inline::Thumbnail)
        } else {
            Err(String::from("parses inline fail."))
        }
    }
}

/// GalleryList structures.
pub struct GalleryList {
    /// ?next=2453493
    pub next: usize,
    /// ?next=2453493&jump=1d
    /// ?next=2453493&jump=3d
    /// ?next=2453493&jump=1w
    /// ?next=2453493&jump=2w
    /// ?next=2453493&jump=1m
    /// ?next=2453493&jump=6m
    /// ?next=2453493&jump=1y
    /// ?next=2453493&jump=2y
    pub jump: Option<String>,
    /// ?next=2453493&seek=2023-02-01
    pub seek: Option<String>,
    pub gallery_info_vec: Vec<GalleryInfo>,
}

impl GalleryList {
    pub fn parse(document: &str) -> Result<GalleryList, String> {
        if let Ok(root) = Vis::load(document) {
            // TODO parse pages
            let next = 2453493 as usize;
            let jump = None;
            let seek = None;

            // Parses gallery info.
            let itg = root.find(".itg");

            // There are several styles of itg.
            let mut items;
            let inline = Inline::parse(&itg)?;
            match inline {
                Inline::MinimalOrMinimalPlus | Inline::Compact => {
                    // Minimal or Minimal+ or Compact.
                    items = itg.children("tr");
                    // First one is header, skip it.
                    items = items.slice(1..);
                }
                Inline::Extended => {
                    // Extended.
                    items = itg.children("tr");
                }
                Inline::Thumbnail => {
                    // Thumbnail.
                    items = itg.find(".gl1t")
                }
            }

            let mut gallery_info_vec = Vec::new();
            for item in items {
                gallery_info_vec.push(GalleryInfo::parse(&item.outer_html(), &inline)?);
            }

            Ok(GalleryList {
                next,
                jump,
                seek,
                gallery_info_vec,
            })
        } else {
            Err(String::from("parses gallery list fail."))
        }
    }
}

fn parse_pages(document: &str) -> Result<usize, String> {
    todo!()
}

fn parse_rating(rating_style: &str) -> Result<f32, String> {
    const PATTERN_RATING: &str = r#"\d+px"#;

    let reg = Regex::new(PATTERN_RATING).unwrap();
    let mut n1 = i32::MIN;
    let mut n2 = i32::MIN;

    let mut rate = 5 as f32;
    let mut ms = reg.find_iter(rating_style);
    if let Some(m) = ms.next() {
        n1 = parse_i32(&m.as_str().replace("px", ""))?;
    }

    if let Some(m) = ms.next() {
        n2 = parse_i32(&m.as_str().replace("px", ""))?;
    }

    if n1 != i32::MIN && n2 != i32::MIN {
        rate -= (n1 / 16) as f32;
        if n2 == 21 {
            rate -= 0.5 as f32;
        }

        Ok(rate)
    } else {
        Err(String::from("parses gallery info rating fail."))
    }
}

fn parse_favorite_slot(style: &str) -> i32 {
    const PATTERN_FAVORITE_SLOT: &str = r#"background-color:rgba\((\d+),(\d+),(\d+),"#;
    let reg = Regex::new(PATTERN_FAVORITE_SLOT).unwrap();

    if reg.is_match(style) {
        let mut ms = reg.find_iter(style);
        let r = ms.nth(1).unwrap().as_str();
        let g = ms.nth(2).unwrap().as_str();
        let b = ms.nth(3).unwrap().as_str();

        let mut slot = 0;
        for rgb in FAVORITE_SLOT_RGB {
            if r == rgb[0] && g == rgb[1] && b == rgb[2] {
                return slot;
            }

            slot += 1;
        }
    }

    -2
}

const PATTERN_NEXT_PAGE: &str = r#"page=(\d+)"#;

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

#[cfg(test)]
mod tests {
    use crate::utils::test::read_test_file;
    use super::*;

    #[test]
    fn parse_pages_test() {}

    #[test]
    fn parse_rating_test() {
        let rating_style = "background-position:0px -21px;opacity:0.53333333333333";
        assert_eq!(parse_rating(rating_style).unwrap(), 4.5 as f32);
    }

    #[test]
    fn parse_gallery_info_test() {
        let document = read_test_file("gallery_list_minimal.html");
        let result = GalleryList::parse(&document);

        let document = read_test_file("gallery_list_compact.html");
        let result = GalleryList::parse(&document);

        let document = read_test_file("gallery_list_extended.html");
        let result = GalleryList::parse(&document);

        let document = read_test_file("gallery_list_thumbnail.html");
        let result = GalleryList::parse(&document);
    }
}
