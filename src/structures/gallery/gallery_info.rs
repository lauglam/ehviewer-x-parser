use regex::Regex;
use visdom::Vis;
use crate::{
    EhResult,
    ParseError,
    Parser,
    utils::{parse_i32, parse_u32},
    structures::{
        Category,
        FavoriteSlot,
        GalleryDetailUrl,
        gallery::InlineSet,
    },
};

#[derive(Debug, PartialEq)]
pub struct GalleryInfo {
    pub gid: u64,
    pub token: String,
    pub title: String,
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
    pub is_favorited: bool,
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}

impl Parser for GalleryInfo {
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc).map_err(|_| String::from("parses gallery info fail."))?;

        let inline_set = root.find(r#".searchnav select[onchange*=inline_set]"#);
        let inline_set = InlineSet::parse(&inline_set.outer_html())?;

        let gl_name = root.find(r#".glname"#);

        // gid, token
        let (gid, token) = (|| -> Result<(u64, String), ParseError> {
            let url = match inline_set {
                InlineSet::Minimal | InlineSet::MinimalPlus | InlineSet::Compact | InlineSet::Thumbnail => {
                    gl_name.find(r#"a"#)
                }
                InlineSet::Extended => {
                    gl_name.parent("a")
                }
            };

            let detail_url = GalleryDetailUrl::parse(&url.text())?;
            Ok((detail_url.gid, detail_url.token))
        })()?;

        // simple_tag_vec_opt
        let simple_tag_vec_opt: Option<Vec<String>> = match inline_set {
            InlineSet::Compact | InlineSet::Extended => {
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
            let cs_or_cn = match inline_set {
                InlineSet::Minimal | InlineSet::MinimalPlus | InlineSet::Thumbnail => {
                    root.find(".gl1m > .cs")
                }
                InlineSet::Compact | InlineSet::Extended => {
                    root.find(".cn")
                }
            };
            Category::from(&cs_or_cn.text()).value
        })();

        // pages
        let pages = (|| -> EhResult<u32>{
            let page = match inline_set {
                InlineSet::Minimal | InlineSet::MinimalPlus | InlineSet::Compact => {
                    root.find(r#".glthumb .ir"#)
                }
                InlineSet::Extended => {
                    root.find(".ir").next("").next("")
                }
                InlineSet::Thumbnail => {
                    root.find(".ir").next("")
                }
            }.text();

            let regex = Regex::new(PATTERN_PAGES).unwrap();
            let captures = regex.captures(&page).unwrap();
            Ok(parse_u32(&captures[1])?)
        })()?;

        // thumb, thumb_height, thumb_width
        let (thumb, thumb_height, thumb_width) = (|| -> Result<(String, u32, u32), ParseError> {
            let img = match inline_set {
                InlineSet::Minimal | InlineSet::MinimalPlus | InlineSet::Compact => {
                    root.find(r#".glthumb img"#)
                }
                InlineSet::Extended | InlineSet::Thumbnail => {
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
        let rating = (|| -> EhResult<f32> {
            let ir = match inline_set {
                InlineSet::Minimal | InlineSet::MinimalPlus | InlineSet::Compact => {
                    root.find(r#".glthumb .ir"#)
                }
                InlineSet::Extended | InlineSet::Thumbnail => {
                    root.find(".ir")
                }
            };
            let style = ir.attr("style").unwrap();
            parse_rating(&style.to_string())
        })()?;

        // posted, is_favorited, favorite_slot, favorite_name_opt
        let (posted, is_favorited, favorite_slot_opt, favorite_name_opt) =
            (|| -> Result<(String, bool, Option<u32>, Option<String>), ParseError>
                {
                    let posted = root.find(&format!("#posted_{}", gid));

                    let (is_favorited, favorite_slot_opt) = if let Some(slot) = posted.attr("style") {
                        (true, Some(FavoriteSlot::parse(&slot.to_string())?.value))
                    } else {
                        (false, None)
                    };

                    let favorite_name_opt = if let Some(title) = posted.attr("title") {
                        Some(title.to_string())
                    } else {
                        None
                    };

                    Ok((posted.text(), is_favorited, favorite_slot_opt, favorite_name_opt))
                })()?;

        // uploader_opt
        let uploader_opt = match inline_set {
            InlineSet::Minimal | InlineSet::MinimalPlus | InlineSet::Compact | InlineSet::Extended => {
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
            is_favorited,
            favorite_slot_opt,
            favorite_name_opt,
            simple_tag_vec_opt,
            simple_language_opt,
        })
    }
}

const PATTERN_THUMB_SIZE: &str = r#"height:(\d+)px;width:(\d+)px"#;
const PATTERN_PAGES: &str = r#"(\d+) page"#;

// const S_LANG_JA: &str = "JA";
// const S_LANG_EN: &str = "EN";
// const S_LANG_ZH: &str = "ZH";
// const S_LANG_NL: &str = "NL";
// const S_LANG_FR: &str = "FR";
// const S_LANG_DE: &str = "DE";
// const S_LANG_HU: &str = "HU";
// const S_LANG_IT: &str = "IT";
// const S_LANG_KO: &str = "KO";
// const S_LANG_PL: &str = "PL";
// const S_LANG_PT: &str = "PT";
// const S_LANG_RU: &str = "RU";
// const S_LANG_ES: &str = "ES";
// const S_LANG_TH: &str = "TH";
// const S_LANG_VI: &str = "VI";

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

fn parse_rating(rating_style: &str) -> EhResult<f32> {
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
        Err(ParseError::RegexMatchFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rating_test() {
        let rating_style = "background-position:0px -21px;opacity:0.53333333333333";
        assert_eq!(parse_rating(rating_style).unwrap(), 4.5 as f32);
    }
}
