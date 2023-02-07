#![allow(dead_code)]

use core::f32;
use regex::Regex;
use visdom::types::Elements;
use visdom::Vis;
use crate::structures::category::Category;
use crate::structures::favorite_slot::FavoriteSlot;
use crate::structures::gallery_detail_url::GalleryDetailUrl;
use crate::utils::{
    parse_i32,
    parse_u32,
};

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
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}

impl GalleryInfo {
    pub fn parse(element: &str, inline: &Inline) -> Result<GalleryInfo, String> {
        const PATTERN_THUMB_SIZE: &str = r#"height:(\d+)px;width:(\d+)px"#;
        const PATTERN_PAGES: &str = r#"(\d+) page"#;

        let root = Vis::load(element).map_err(|_| String::from("parses gallery info fail."))?;
        let gl_name = root.find(r#".glname"#);

        // gid, token
        let (gid, token) = (|| -> Result<(u64, String), String> {
            let url = match inline {
                Inline::MinimalOrMinimalPlus | Inline::Compact | Inline::Thumbnail => {
                    gl_name.find(r#"a"#)
                }
                Inline::Extended => {
                    gl_name.parent("a")
                }
            };

            let detail_url = GalleryDetailUrl::parse(&url.text(), true)?;
            Ok((detail_url.gid, detail_url.token))
        })()?;

        // simple_tag_vec_opt
        let simple_tag_vec_opt: Option<Vec<String>> = match inline {
            Inline::Compact | Inline::Extended => {
                let gts = gl_name.find(".gt");
                let mut simple_tag_vec = Vec::new();
                for gt in gts {
                    let title_attr = gt.get_attribute("title").unwrap();
                    simple_tag_vec.push(title_attr.to_string())
                }

                Some(simple_tag_vec)
            }
            _ => None,
        };

        // category
        let category = (|| {
            let cs_or_cn = match inline {
                Inline::MinimalOrMinimalPlus | Inline::Thumbnail => {
                    root.find(".gl1m > .cs")
                }
                Inline::Compact | Inline::Extended => {
                    root.find(".cn")
                }
            };
            Category::from(&cs_or_cn.text()).value
        })();

        // pages
        let pages = (|| -> Result<u32, String>{
            let page = match inline {
                Inline::MinimalOrMinimalPlus | Inline::Compact => {
                    root.find(r#".glthumb .ir"#)
                }
                Inline::Extended => {
                    root.find(".ir").next("").next("")
                }
                Inline::Thumbnail => {
                    root.find(".ir").next("")
                }
            }.text();

            let regex = Regex::new(PATTERN_PAGES).unwrap();
            let captures = regex.captures(&page).unwrap();
            Ok(parse_u32(&captures[1])?)
        })()?;

        // thumb, thumb_height, thumb_width
        let (thumb, thumb_height, thumb_width) = (|| -> Result<(String, u32, u32), String> {
            let img = match inline {
                Inline::MinimalOrMinimalPlus | Inline::Compact => {
                    root.find(r#".glthumb img"#)
                }
                Inline::Extended | Inline::Thumbnail => {
                    root.find("img")
                }
            };

            let style = img.attr("style").unwrap().to_string();
            let regex = Regex::new(PATTERN_THUMB_SIZE).unwrap();
            let captures = regex.captures(&style).unwrap();

            // TODO setting
            Ok((
                img.attr("src").unwrap().to_string(),
                parse_u32(&captures[1])?,
                parse_u32(&captures[2])?,
            ))
        })()?;

        // rating
        let rating = (|| -> Result<f32, String> {
            let ir = match inline {
                Inline::MinimalOrMinimalPlus | Inline::Compact => {
                    root.find(r#".glthumb .ir"#)
                }
                Inline::Extended | Inline::Thumbnail => {
                    root.find(".ir")
                }
            };
            let style = ir.attr("style").unwrap();
            parse_rating(&style.to_string())
        })()?;

        // posted, favorite_slot
        let (posted, favorite_slot_opt) = (|| -> Result<(String, Option<u32>), String>{
            let posted = root.find(&format!("#posted_{}", gid));
            let favorite_slot_opt = if let Some(slot) = posted.attr("style") {
                Some(FavoriteSlot::parse(&slot.to_string())?.value)
            } else {
                None
            };
            Ok((posted.text(), favorite_slot_opt))
        })()?;

        // uploader_opt
        let uploader_opt = match inline {
            Inline::MinimalOrMinimalPlus | Inline::Compact | Inline::Extended => {
                let prefix = r#"https://e-hentai.org/uploader/"#;
                let uploader = root.find(&format!("[href^={}]", prefix));
                Some(uploader.text())
            }
            _ => None,
        };

        // title
        let title = root.find(r#".glink"#).text();

        // simple_language_opt
        let simple_language_opt: Option<String> = match simple_tag_vec_opt {
            Some(ref simple_tag_vec) => {
                let mut result = None;
                for tag in simple_tag_vec {
                    let idx_opt = S_LANG_TAGS.iter().position(|&t| t == tag);
                    if let Some(idx) = idx_opt {
                        result = Some(String::from(S_LANGS[idx]));
                        break;
                    }
                }

                result
            }
            None => {
                let idx_opt = S_LANG_PATTERNS.iter().position(|pattern| {
                    let regex = Regex::new(pattern).unwrap();
                    regex.is_match(&title)
                });

                let mut result = None;
                if let Some(idx) = idx_opt {
                    result = Some(String::from(S_LANGS[idx]));
                }

                result
            }
        };

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
            favorite_slot_opt,
            simple_tag_vec_opt,
            simple_language_opt,
            title_jpn_opt: None,
            favorite_name_opt: None,
        })
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
    pub fn parse(doc: &str) -> Result<GalleryList, String> {
        if let Ok(root) = Vis::load(doc) {
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

fn parse_pages(doc: &str) -> Result<usize, String> {
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

const PATTERN_NEXT_PAGE: &str = r#"page=(\d+)"#;

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
        let doc = read_test_file("gallery_list_minimal.html");
        let result = GalleryList::parse(&doc);

        let doc = read_test_file("gallery_list_compact.html");
        let result = GalleryList::parse(&doc);

        let doc = read_test_file("gallery_list_extended.html");
        let result = GalleryList::parse(&doc);

        let doc = read_test_file("gallery_list_thumbnail.html");
        let result = GalleryList::parse(&doc);
    }
}
