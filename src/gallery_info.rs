#![allow(dead_code)]

use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct GalleryInfo {
    pub gid: u64,
    pub token: String,
    pub title: String,
    pub title_jpn: Option<String>,
    pub thumb: String,
    pub category: usize,
    pub posted: String,
    pub uploader: String,
    pub rating: f32,
    pub rated: bool,
    pub simple_tags: Option<Vec<String>>,
    pub pages: usize,
    pub thumb_width: usize,
    pub thumb_height: usize,
    pub span_size: usize,
    pub span_index: usize,
    pub span_group_index: usize,
    pub simple_language: Option<String>,
    pub favorite_name: String,
    favorite_slot: Option<isize>,
}


impl GalleryInfo {
    pub fn available_title(&self) -> &str {
        match self.title_jpn {
            Some(ref title_jpn) => title_jpn,
            // todo: title is option?
            None => self.title.as_ref(),
        }
    }

    pub fn favorite_slot(&self) -> isize {
        match self.favorite_slot {
            Some(favorite_slot) => favorite_slot,
            None => -2,
        }
    }

    pub fn generate_s_lang(&mut self) {
        if self.simple_tags.is_some() {
            self.generate_s_lang_from_tags();
        }

        // todo: the simple_language is none, do this?
        if self.simple_language.is_none() {
            self.generate_s_lang_from_title();
        }
    }

    fn generate_s_lang_from_tags(&mut self) {
        if self.simple_tags.is_none() {
            return;
        }

        for tag in self.simple_tags.as_ref().unwrap() {
            for index in 0..S_LANG_S.len() {
                if S_LANG_TAGS[index] == tag {
                    self.simple_language = Some(String::from(S_LANG_S[index]));
                    return;
                }
            }
        }
    }

    fn generate_s_lang_from_title(&mut self) {
        for index in 0..S_LANG_S.len() {
            let regex = Regex::new(S_LANG_PATTERNS[index]).unwrap();
            if regex.is_match(&self.title) {
                self.simple_language = Some(String::from(S_LANG_S[index]));
                return;
            }
        }

        self.simple_language = None;
    }
}

impl ToString for GalleryInfo {
    fn to_string(&self) -> String {
        match self.title_jpn {
            Some(ref title_jpn) => format!("{} {}", self.gid, title_jpn),
            None => format!("{} {}", self.gid, self.title),
        }
    }
}

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

const S_LANG_S: [&str; 14] = [
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
