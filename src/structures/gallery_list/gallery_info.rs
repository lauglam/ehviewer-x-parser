//! 1. Minimal
//! 2. MinimalPlus
//! 3. Compact
//! 4. Extended
//! 5. Thumbnail

use regex::Regex;
use visdom::{Vis, types::Elements};
use crate::{
    EhResult,
    Parser,
    REGEX_MATCH_FAILED,
    ATTRIBUTE_NOT_FOUND,
    structures::{
        Thumb,
        Rating,
        Category,
        FavoriteSlot,
        GalleryDetailUrl,
    },
};

#[derive(Debug, PartialEq)]
pub struct GalleryInfoMinimal {
    pub gid: u64,
    pub token: String,
    pub title: String,
    pub thumb: Thumb,
    pub category: u32,
    pub posted: String,
    /// Minimal MinimalPlus Compact Extended
    pub uploader: String,
    pub rating: f32,
    pub pages: u32,
    pub simple_language_opt: Option<String>,
    pub is_favorited: bool,
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}

impl Parser for GalleryInfoMinimal {
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;
        let (gid, token) = parse_gid_and_token_1_2_3_5(&root)?;
        let category = parse_category_1_2_5(&root)?;
        let pages = parse_pages_1_2_3(&root)?;
        let thumb = parse_thumb_1_2_3(&root)?;
        let rating = parse_rating_1_2_3(&root)?;
        let posted = parse_posted(&root)?;
        let is_favorited = parse_is_favorited(&root)?;
        let favorite_slot_opt = parse_favorite_slot_opt(&root)?;
        let favorite_name_opt = parse_favorite_name_opt(&root)?;
        let uploader = parse_uploader_1_2_3_4(&root)?;
        let title = parse_title(&root)?;
        let simple_language_opt = parse_simple_language_opt_1_2_5(&root)?;

        Ok(GalleryInfoMinimal {
            gid,
            token,
            title,
            thumb,
            category,
            posted,
            uploader,
            rating,
            pages,
            simple_language_opt,
            is_favorited,
            favorite_slot_opt,
            favorite_name_opt,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct GalleryInfoMinimalPlus {
    pub gid: u64,
    pub token: String,
    pub title: String,
    pub thumb: Thumb,
    pub category: u32,
    pub posted: String,
    pub uploader: String,
    pub rating: f32,
    pub pages: u32,
    pub simple_language_opt: Option<String>,
    pub is_favorited: bool,
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}

impl From<GalleryInfoMinimal> for GalleryInfoMinimalPlus {
    fn from(value: GalleryInfoMinimal) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Parser for GalleryInfoMinimalPlus {
    fn parse(doc: &str) -> EhResult<Self> {
        Ok(GalleryInfoMinimal::parse(doc)?.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct GalleryInfoCompact {
    pub gid: u64,
    pub token: String,
    pub title: String,
    pub thumb: Thumb,
    pub category: u32,
    pub posted: String,
    /// Minimal MinimalPlus Compact Extended
    pub uploader: String,
    pub rating: f32,
    /// Compact Extended
    pub simple_tag_vec: Vec<String>,
    pub pages: u32,
    pub simple_language_opt: Option<String>,
    pub is_favorited: bool,
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}

impl Parser for GalleryInfoCompact {
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;
        let (gid, token) = parse_gid_and_token_1_2_3_5(&root)?;
        let category = parse_category_3_4(&root)?;
        let pages = parse_pages_1_2_3(&root)?;
        let thumb = parse_thumb_1_2_3(&root)?;
        let rating = parse_rating_1_2_3(&root)?;
        let posted = parse_posted(&root)?;
        let is_favorited = parse_is_favorited(&root)?;
        let favorite_slot_opt = parse_favorite_slot_opt(&root)?;
        let favorite_name_opt = parse_favorite_name_opt(&root)?;
        let uploader = parse_uploader_1_2_3_4(&root)?;
        let title = parse_title(&root)?;
        let simple_language_opt = parse_simple_language_opt_3_4(&root)?;
        let simple_tag_vec = parse_simple_tag_vec_3_4(&root)?;

        Ok(GalleryInfoCompact {
            gid,
            token,
            title,
            thumb,
            category,
            posted,
            uploader,
            rating,
            simple_tag_vec,
            pages,
            simple_language_opt,
            is_favorited,
            favorite_slot_opt,
            favorite_name_opt,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct GalleryInfoExtended {
    pub gid: u64,
    pub token: String,
    pub title: String,
    pub thumb: Thumb,
    pub category: u32,
    pub posted: String,
    /// Minimal MinimalPlus Compact Extended
    pub uploader: String,
    pub rating: f32,
    /// Compact Extended
    pub simple_tag_vec: Vec<String>,
    pub pages: u32,
    pub simple_language_opt: Option<String>,
    pub is_favorited: bool,
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}

impl Parser for GalleryInfoExtended {
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;
        let (gid, token) = parse_gid_and_token_4(&root)?;
        let category = parse_category_3_4(&root)?;
        let pages = parse_pages_4(&root)?;
        let thumb = parse_thumb_4_5(&root)?;
        let rating = parse_rating_4_5(&root)?;
        let posted = parse_posted(&root)?;
        let is_favorited = parse_is_favorited(&root)?;
        let favorite_slot_opt = parse_favorite_slot_opt(&root)?;
        let favorite_name_opt = parse_favorite_name_opt(&root)?;
        let uploader = parse_uploader_1_2_3_4(&root)?;
        let title = parse_title(&root)?;
        let simple_language_opt = parse_simple_language_opt_3_4(&root)?;
        let simple_tag_vec = parse_simple_tag_vec_3_4(&root)?;

        Ok(GalleryInfoExtended {
            gid,
            token,
            title,
            thumb,
            category,
            posted,
            uploader,
            rating,
            simple_tag_vec,
            pages,
            simple_language_opt,
            is_favorited,
            favorite_slot_opt,
            favorite_name_opt,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct GalleryInfoThumbnail {
    pub gid: u64,
    pub token: String,
    pub title: String,
    pub thumb: Thumb,
    pub category: u32,
    pub posted: String,
    pub rating: f32,
    pub pages: u32,
    pub simple_language_opt: Option<String>,
    pub is_favorited: bool,
    pub favorite_slot_opt: Option<u32>,
    pub favorite_name_opt: Option<String>,
}

impl Parser for GalleryInfoThumbnail {
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;
        let (gid, token) = parse_gid_and_token_1_2_3_5(&root)?;
        let category = parse_category_1_2_5(&root)?;
        let pages = parse_pages_5(&root)?;
        let thumb = parse_thumb_4_5(&root)?;
        let rating = parse_rating_4_5(&root)?;
        let posted = parse_posted(&root)?;
        let is_favorited = parse_is_favorited(&root)?;
        let favorite_slot_opt = parse_favorite_slot_opt(&root)?;
        let favorite_name_opt = parse_favorite_name_opt(&root)?;
        let title = parse_title(&root)?;
        let simple_language_opt = parse_simple_language_opt_1_2_5(&root)?;

        Ok(GalleryInfoThumbnail {
            gid,
            token,
            title,
            thumb,
            category,
            posted,
            rating,
            pages,
            simple_language_opt,
            is_favorited,
            favorite_slot_opt,
            favorite_name_opt,
        })
    }
}

fn parse_gid_and_token_1_2_3_5(root: &Elements) -> EhResult<(u64, String)> {
    let a = root.find(r#".glname a"#);
    let href = a.attr("href").ok_or(ATTRIBUTE_NOT_FOUND)?;
    let detail_url = GalleryDetailUrl::parse(&href.to_string())?;
    Ok((detail_url.gid, detail_url.token))
}

fn parse_gid_and_token_4(root: &Elements) -> EhResult<(u64, String)> {
    let gl_name = root.find(r#".glname"#);
    let a = gl_name.parent("a");
    let detail_url = GalleryDetailUrl::parse(&a.text())?;
    Ok((detail_url.gid, detail_url.token))
}

fn parse_simple_tag_vec_3_4(root: &Elements) -> EhResult<Vec<String>> {
    let gts = root.find(r#".glname .gt"#);
    let mut simple_tag_vec = Vec::new();
    for gt in gts {
        let title_attr = gt.get_attribute("title").ok_or(ATTRIBUTE_NOT_FOUND)?;
        simple_tag_vec.push(title_attr.to_string());
    }

    Ok(simple_tag_vec)
}

fn parse_category_1_2_5(root: &Elements) -> EhResult<u32> {
    let cs = root.find(".gl1m > .cs");
    let category = Category::from(&cs.text());

    Ok(category.value)
}

fn parse_category_3_4(root: &Elements) -> EhResult<u32> {
    let cn = root.find(".cn");
    let category = Category::from(&cn.text());

    Ok(category.value)
}

fn parse_pages_1_2_3(root: &Elements) -> EhResult<u32> {
    let ir = root.find(r#".glthumb .ir"#);

    let sibling = ir.siblings("div");
    let sibling_str = sibling.text();

    let regex = Regex::new(PATTERN_PAGES).unwrap();
    let captures = regex.captures(&sibling_str).ok_or(REGEX_MATCH_FAILED)?;

    Ok(captures[1].parse()?)
}

fn parse_pages_4(root: &Elements) -> EhResult<u32> {
    let ir = root.find(".ir");
    let sibling = ir.next("").next("").text();

    let regex = Regex::new(PATTERN_PAGES).unwrap();
    let captures = regex.captures(&sibling).ok_or(REGEX_MATCH_FAILED)?;

    Ok(captures[1].parse()?)
}

fn parse_pages_5(root: &Elements) -> EhResult<u32> {
    let ir = root.find(".ir");
    let sibling = ir.next("").text();

    let regex = Regex::new(PATTERN_PAGES).unwrap();
    let captures = regex.captures(&sibling).ok_or(REGEX_MATCH_FAILED)?;

    Ok(captures[1].parse()?)
}

fn parse_thumb_1_2_3(root: &Elements) -> EhResult<Thumb> {
    let img = root.find(r#".glthumb img"#);
    Ok(Thumb::parse(&img.outer_html())?)
}

fn parse_thumb_4_5(root: &Elements) -> EhResult<Thumb> {
    let img = root.find("img");
    Ok(Thumb::parse(&img.outer_html())?)
}

fn parse_rating_1_2_3(root: &Elements) -> EhResult<f32> {
    let ir = root.find(r#".glthumb .ir"#);
    let style = ir.attr("style").ok_or(ATTRIBUTE_NOT_FOUND)?;
    let rating = Rating::parse(&style.to_string())?;

    Ok(rating.value)
}

fn parse_rating_4_5(root: &Elements) -> EhResult<f32> {
    let ir = root.find(".ir");
    let style = ir.attr("style").ok_or(ATTRIBUTE_NOT_FOUND)?;
    let rating = Rating::parse(&style.to_string())?;

    Ok(rating.value)
}

fn parse_posted(root: &Elements) -> EhResult<String> {
    let posted = root.find("[id^=posted_]");
    Ok(posted.text())
}

fn parse_is_favorited(root: &Elements) -> EhResult<bool> {
    let posted = root.find("[id^=posted_]");
    Ok(posted.attr("style").is_some())
}


fn parse_favorite_slot_opt(root: &Elements) -> EhResult<Option<u32>> {
    let posted = root.find("[id^=posted_]");
    if let Some(slot) = posted.attr("style") {
        let slot = FavoriteSlot::parse(&slot.to_string())?;
        Ok(Some(slot.value))
    } else {
        Ok(None)
    }
}

fn parse_favorite_name_opt(root: &Elements) -> EhResult<Option<String>> {
    let posted = root.find("[id^=posted_]");
    if let Some(title) = posted.attr("title") {
        Ok(Some(title.to_string()))
    } else {
        Ok(None)
    }
}

fn parse_uploader_1_2_3_4(root: &Elements) -> EhResult<String> {
    let prefix = r#"https://e-hentai.org/uploader/"#;
    let a = root.find(&format!("[href^={}]", prefix));

    Ok(a.text())
}

fn parse_title(root: &Elements) -> EhResult<String> {
    let link = root.find(r#".glink"#);
    Ok(link.text())
}

fn parse_simple_language_opt_1_2_5(root: &Elements) -> EhResult<Option<String>> {
    let link = root.find(r#".glink"#);
    let idx_opt = S_LANG_PATTERNS.iter().position(|pattern| {
        let regex = Regex::new(pattern).unwrap();
        regex.is_match(&link.text())
    });

    let mut simple_language_opt = None;
    if let Some(idx) = idx_opt {
        simple_language_opt = Some(String::from(S_LANGS[idx]));
    }

    Ok(simple_language_opt)
}

fn parse_simple_language_opt_3_4(root: &Elements) -> EhResult<Option<String>> {
    let mut simple_language_opt = None;
    let simple_tag_vec = parse_simple_tag_vec_3_4(root)?;

    for tag in simple_tag_vec {
        let idx_opt = S_LANG_TAGS.iter().position(|&t| t == tag);
        if let Some(idx) = idx_opt {
            simple_language_opt = Some(String::from(S_LANGS[idx]));
            break;
        }
    }
    Ok(simple_language_opt)
}

const PATTERN_PAGES: &str = r#"(\d+) page"#;

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

// const PATTERN_THUMB_SIZE: &str = r#"height:(\d+)px;width:(\d+)px"#;

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
