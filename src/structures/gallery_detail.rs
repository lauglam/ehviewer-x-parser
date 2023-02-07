#![allow(dead_code)]

use regex::Regex;
use visdom::Vis;
use std::collections::HashMap;
use crate::structures::category::Category;
use crate::structures::favorite_slot::FavoriteSlot;
use crate::structures::gallery_detail_url::GalleryDetailUrl;
use crate::utils::{
    parse_f32,
    parse_u32,
    parse_u64,
    unescape,
};

#[derive(Debug, PartialEq)]
pub struct Comment {}

#[derive(Debug, PartialEq)]
pub struct PreviewSet {}

#[derive(Debug, PartialEq)]
pub struct GalleryDetailDetail {
    pub posted: String,
    pub parent_opt: Option<String>,
    pub visible: String,
    pub language: String,
    pub file_size: String,
    pub pages: u32,
    pub favorite_count: u32,
}

impl GalleryDetailDetail {
    /// <table>
    ///     <tr>
    ///         <td class="gdt1">Posted:</td>
    ///         <td class="gdt2">2023-02-07 07:33</td>
    ///     </tr>
    ///     ...
    /// </table>
    pub fn parse(table: &str) -> Result<GalleryDetailDetail, String> {
        const PATTERN_PAGES: &str = r#"(\d+) pages"#;
        const PATTERN_FAVORITE_COUNT: &str = r#"(\d+) times"#;

        if let Ok(root) = Vis::load(table) {
            let gdt1s = root.find(".gdt1");

            let (
                mut posted,
                mut parent_opt,
                mut visible,
                mut language,
                mut file_size,
                mut pages,
                mut favorite_count
            ) = (None, None, None, None, None, None, None);

            for gdt1 in gdt1s {
                match gdt1.text().as_str() {
                    "Posted:" => {
                        let gdt2 = gdt1.next_element_sibling().unwrap();
                        posted = Some(gdt2.text());
                    }
                    "Parent:" => {
                        let gdt2 = gdt1.next_element_sibling().unwrap();

                        if let Some(href) = gdt2.get_attribute("href") {
                            parent_opt = Some(href.to_string());
                        }
                    }
                    "Visible:" => {
                        let gdt2 = gdt1.next_element_sibling().unwrap();
                        visible = Some(gdt2.text());
                    }
                    "Language:" => {
                        let gdt2 = gdt1.next_element_sibling().unwrap();
                        language = Some(gdt2.text());
                    }
                    "File Size:" => {
                        let gdt2 = gdt1.next_element_sibling().unwrap();
                        file_size = Some(gdt2.text());
                    }
                    "Length:" => {
                        let gdt2 = gdt1.next_element_sibling().unwrap();
                        let gdt2 = gdt2.text();

                        let regex = Regex::new(PATTERN_PAGES).unwrap();
                        let captures = regex.captures(&gdt2).unwrap();
                        pages = Some(parse_u32(&captures[1])?);
                    }
                    "Favorited:" => {
                        let gdt2 = gdt1.next_element_sibling().unwrap();
                        let gdt2 = gdt2.text();

                        let regex = Regex::new(PATTERN_FAVORITE_COUNT).unwrap();
                        let captures = regex.captures(&gdt2).unwrap();
                        favorite_count = Some(parse_u32(&captures[1])?);
                    }
                    _ => {}
                }
            }

            if let (
                Some(posted),
                Some(visible),
                Some(language),
                Some(file_size),
                Some(pages),
                Some(favorite_count)
            ) = (posted, visible, language, file_size, pages, favorite_count) {
                return Ok(GalleryDetailDetail {
                    posted,
                    parent_opt,
                    visible,
                    language,
                    file_size,
                    pages,
                    favorite_count,
                });
            }
        }

        Err(String::from("parses gallery detail detail fail."))
    }
}

#[derive(Debug, PartialEq)]
pub struct GalleryDetail {
    pub gid: u64,
    pub token: String,
    pub api_uid: u64,
    pub api_key: String,
    pub torrent_count: u32,
    pub torrent_url: String,
    pub archive_url: String,
    pub thumb: String,
    pub newer_version_map_opt: Option<HashMap<String, GalleryDetailUrl>>,
    pub is_favorited: bool,
    pub favorite_name_opt: Option<String>,
    pub favorite_slot_opt: Option<u32>,
    pub rating_count: u32,
    pub tag_group_vec: Vec<GalleryTagGroup>,
    pub comment_vec: Vec<Comment>,
    pub preview_pages: u32,
    pub preview_set: PreviewSet,
    pub url: String,
    pub title: String,
    pub title_jpn: String,
    pub category: u32,
    pub uploader: String,
    pub rating_opt: Option<f32>,
    pub detail: GalleryDetailDetail,
}

impl GalleryDetail {
    pub fn parse(doc: &str) -> Result<GalleryDetail, String> {
        const OFFENSIVE_STRING: &str = "<p>(And if you choose to ignore this warning, you lose all rights to complain about it in the future.)</p>";
        const PINING_STRING: &str = "<p>This gallery is pining for the fjords.</p>";

        if doc.contains(OFFENSIVE_STRING) {
            return Err(String::from("if you choose to ignore this warning, you lose all rights to complain about it in the future."));
        }

        if doc.contains(PINING_STRING) {
            return Err(String::from("this gallery is pining for the fjords."));
        }

        // Error info.
        const PATTERN_ERROR: &str = "<div class=\"d\">\n<p>([^<]+)</p>";
        let regex = Regex::new(PATTERN_ERROR).unwrap();
        if let Some(cap) = regex.captures(doc) {
            return Err(String::from(&cap[1]));
        }


        todo!();
    }
}

fn parse_internal(doc: &str) -> Option<GalleryDetail> {
    const PATTERN_DETAIL: &str = "var gid = (\\d+);\\s*?(\n|\r|\r\n)?\\s*?var token = \"([a-f0-9]+)\";\\s*?(\n|\r|\r\n)?\\s*?var apiuid = ([\\-\\d]+);\\s*?(\n|\r|\r\n)?\\s*?var apikey = \"([a-f0-9]+)\";";
    const PATTERN_TORRENT: &str = r#"<a[^<>]*onclick="return popUp\('([^']+)'[^)]+\)">Torrent Download[^<]+(\d+)[^<]+</a"#;
    const PATTERN_ARCHIVE: &str = r#"<a[^<>]*onclick="return popUp\('([^']+)'[^)]+\)">Archive Download</a>"#;
    const PATTERN_RATING: &str = r#"[+-]?([0-9]*[.]?[0-9]+)"#;
    const PATTERN_NEWER_DATE: &str = ", added (.+?)<br />";

    let regex = Regex::new(PATTERN_DETAIL).unwrap();
    let captures = regex.captures(doc)?;
    let gid = parse_u64(&captures[1]).ok()?;
    let api_uid = parse_u64(&captures[5]).ok()?;
    let token = String::from(&captures[3]);
    let api_key = String::from(&captures[7]);

    let regex = Regex::new(PATTERN_TORRENT).unwrap();
    let captures = regex.captures(doc)?;
    let torrent_url = String::from(unescape(&captures[1]));
    let torrent_count = parse_u32(&captures[2]).ok()?;

    let regex = Regex::new(PATTERN_ARCHIVE).unwrap();
    let captures = regex.captures(doc)?;
    let archive_url = String::from(unescape(&captures[1]));

    let root = Vis::load(doc).ok()?;
    let gm = root.find(".gm:not(#cdiv)");

    let cover = gm.find("#gd1 div:first-child");
    let cover_style = cover.attr("style")?;
    let thumb = parse_cover_style(&cover_style.to_string()).ok()?;

    let gn = gm.find("#gn");
    let title = gn.text();

    let gj = gm.find("#gj");
    let title_jpn = gj.text();

    let cs = gm.find("#gbc > .cs");
    let category = Category::from(&cs.text()).value;

    let gdn = gm.find("#gdn");
    let uploader = gdn.text();

    let gdd = gm.find("#gdd");
    let detail = GalleryDetailDetail::parse(&gdd.html()).ok()?;

    let rat = gm.find("#rating_count");
    let rating_count = parse_u32(&rat.text()).ok()?;

    let label = gm.find("#rating_label");
    let label_text = label.text();
    let mut rating_opt: Option<f32> = None;
    if label_text != "Not Yet Rated" {
        let regex = Regex::new(PATTERN_RATING).unwrap();
        let captures = regex.captures(&label_text)?;
        rating_opt = Some(parse_f32(&captures[1]).ok()?);
    }

    let gdf = gm.find("#gdf");
    let favorite_link = gdf.find("#favoritelink");
    let is_favorited = !favorite_link.text().contains("Add to Favorites");

    let (favorite_slot_opt, favorite_name_opt) = if is_favorited {
        let i = gdf.find(".i");
        let style = i.attr("style")?;
        let favorite_slot = FavoriteSlot::parse(&style.to_string()).ok()?.value;

        (Some(favorite_slot), Some(favorite_link.text()))
    } else {
        (None, None)
    };

    let gnd = root.find("#gnd");
    let newer_version_map_opt = if !gnd.is_empty() {
        let regex = Regex::new(PATTERN_NEWER_DATE).unwrap();
        let date_vec = regex.captures_iter(doc)
            .map(|cap| String::from(&cap[1]))
            .collect::<Vec<String>>();

        let mut newer_version_map = HashMap::new();
        let hrefs = gnd.find("a");
        for (idx, href) in hrefs.into_iter().enumerate() {
            let href = href.get_attribute("href")?;
            let detail_url = GalleryDetailUrl::parse(&href.to_string(), true).ok()?;
            newer_version_map.insert(date_vec[idx].clone(), detail_url);
        }
        Some(newer_version_map)
    } else {
        None
    };

    let cdiv = root.find("#cdiv");



    Some(GalleryDetail {
        gid,
        token,
        api_uid,
        api_key,
        torrent_count,
        torrent_url,
        archive_url,
        thumb,
        newer_version_map_opt,
        is_favorited,
        favorite_slot_opt,
        favorite_name_opt,
        rating_count,
        tag_group_vec: vec![],
        comment_vec: vec![],
        preview_pages: 0,
        preview_set: PreviewSet {},
        url: "".to_string(),
        title,
        title_jpn,
        category,
        uploader,
        rating_opt,
        detail,
    })
}

fn parse_cover_style(style: &str) -> Result<String, String> {
    const PATTERN_COVER: &str = r#"width:(\d+)px; height:(\d+)px.+?url\((.+?)\)"#;

    let regex = Regex::new(PATTERN_COVER).unwrap();
    if let Some(cap) = regex.captures(style) {
        Ok(String::from(&cap[3]))
    } else {
        Err(String::from("parses cover style fail."))
    }
}

#[derive(Debug, PartialEq)]
pub struct GalleryTagGroup {
    pub tag_group_name: String,
    pub tag_vec: Vec<String>,
}

impl GalleryTagGroup {
    /// ```html
    /// <tr><td class="tc">parody:</td><td><div class="gtl" title="parody:senran kagura">senran kagura</div><div class="gtl" title="parody:the idolmaster">the idolmaster</div></td></tr>
    ///                    ^                                                             ^                                                                 ^
    ///                    tag_group_name                                                tag_vec[0]                                                        tag_vec[1]
    /// ```
    pub fn parse(element: &str) -> Result<GalleryTagGroup, String> {
        if let Ok(root) = Vis::load(element) {
            let tag_group_name = root.find(".tc").text();
            let tag_group_name = String::from(&tag_group_name[..tag_group_name.len() - 1]);

            let tag_vec = root.find(".gtl")
                .map(|_, ele| {
                    return ele.text();
                });

            if !tag_group_name.is_empty() && !tag_vec.is_empty() {
                return Ok(GalleryTagGroup {
                    tag_group_name,
                    tag_vec,
                });
            }
        }

        Err(String::from("parses gallery tag group fail."))
    }

    pub fn size(&self) -> usize {
        self.tag_vec.len()
    }

    pub fn get_tag_at(&self, index: usize) -> Option<&String> {
        self.tag_vec.get(index)
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tag_vec.push(tag)
    }
}

impl ToString for GalleryTagGroup {
    fn to_string(&self) -> String {
        format!("{} ({})", self.tag_group_name, self.size())
    }
}

// Regex.

const PATTERN_COMMENT_DATETIME: &str = r#"Posted\s*on\s*(.+?)\s*by"#;
const PATTERN_TAG_GROUP: &str = r#"<tr><td[^<>]+>([\w\s]+):</td><td>(?:<div[^<>]+><a[^<>]+>[\w\s]+</a></div>)+</td></tr>"#;
const PATTERN_TAG: &str = r#"<div[^<>]+><a[^<>]+>([\w\s]+)</a></div>"#;
const PATTERN_COMMENT: &str = r#"<div class="c3">Posted on ([^<>]+) by: &nbsp; <a[^<>]+>([^<>]+)</a>.+?<div class="c6"[^>]*>(.+?)</div><div class="c[78]""#;
const PATTERN_PAGES: &str = r#"<tr><td[^<>]*>Length:</td><td[^<>]*>([\d,]+) pages</td></tr>"#;
const PATTERN_PREVIEW_PAGES: &str = r#"<td[^>]+><a[^>]+>([\d,]+)</a></td><td[^>]+>(?:<a[^>]+>)?&gt;(?:</a>)?</td>"#;
const PATTERN_NORMAL_PREVIEW: &str = r#"<div class="gdtm"[^<>]*><div[^<>]*width:(\d+)[^<>]*height:(\d+)[^<>]*\((.+?)\)[^<>]*-(\d+)px[^<>]*><a[^<>]*href="(.+?)"[^<>]*><img alt="([\d,]+)""#;
const PATTERN_LARGE_PREVIEW: &str = r#"<div class="gdtl".+?<a href="(.+?)"><img alt="([\d,]+)".+?src="(.+?)""#;

#[cfg(test)]
mod tests {
    use crate::utils::test::read_test_file;
    use super::*;

    #[test]
    fn parse_tag_group_test() {
        let element = r#"
            <tr>
                <td class="tc">parody:</td>
                <td>
                    <div class="gtl" title="parody:senran kagura">senran kagura</div>
                    <div class="gtl" title="parody:the idolmaster">the idolmaster</div>
                </td>
            </tr>"#;

        let tag_group = GalleryTagGroup::parse(&element).unwrap();
        assert_eq!(tag_group.tag_vec, vec![r#"senran kagura"#, r#"the idolmaster"#]);
        assert_eq!(tag_group.tag_group_name, r#"parody"#);
    }

    #[test]
    fn parse_test() {
        let doc = read_test_file("gallery_detail.html");
        parse_internal(&doc);
    }

    #[test]
    fn parse_cover_style_test() {
        let style = r#"width:250px; height:354px; background:transparent url(https://ehgt.org/8f/3e/8f3ed3234614db3932038b8d7c80a6fd17fe2c41-2942019-2828-4000-jpg_250.jpg) no-repeat"#;
        assert_eq!(parse_cover_style(style).unwrap(), r#"https://ehgt.org/8f/3e/8f3ed3234614db3932038b8d7c80a6fd17fe2c41-2942019-2828-4000-jpg_250.jpg"#);
    }

    #[test]
    fn parse_detail_test() {
        let table = r#"
        <table>
            <tr>
                <td class="gdt1">Posted:</td>
                <td class="gdt2">2023-02-07 07:33</td>
            </tr>
            <tr>
                <td class="gdt1">Parent:</td>
                <td class="gdt2">None</td>
            </tr>
            <tr>
                <td class="gdt1">Visible:</td>
                <td class="gdt2">Yes</td>
            </tr>
            <tr>
                <td class="gdt1">Language:</td>
                <td class="gdt2">Japanese &nbsp;</td>
            </tr>
            <tr>
                <td class="gdt1">File Size:</td>
                <td class="gdt2">225.5 MB</td>
            </tr>
            <tr>
                <td class="gdt1">Length:</td>
                <td class="gdt2">75 pages</td>
            </tr>
            <tr>
                <td class="gdt1">Favorited:</td>
                <td class="gdt2" id="favcount">23 times</td>
            </tr>
        </table>
        "#;

        assert_eq!(GalleryDetailDetail::parse(table).is_ok(), true);
    }
}
