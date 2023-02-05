#![allow(dead_code)]

use regex::Regex;
use visdom::Vis;
use crate::parser::gallery_list_parser::GalleryInfo;

pub struct Comment {}

pub struct PreviewSet {}

pub struct GalleryDetail {
    pub api_uid: u64,
    pub api_key: String,
    pub torrent_count: usize,
    pub torrent_url: String,
    pub archive_url: String,
    pub parent: String,
    pub newer_versions: Vec<GalleryInfo>,
    pub visible: String,
    pub language: String,
    pub size: String,
    pub favorite_count: usize,
    pub is_favorite: bool,
    pub rating_count: usize,
    pub tag_group_vec: Vec<GalleryTagGroup>,
    pub comment_vec: Vec<Comment>,
    pub preview_pages: usize,
    pub preview_set: PreviewSet,
    pub url: String,
}

impl GalleryDetail {
    pub fn parse(document: &str) -> Result<GalleryDetail, String> {
        const OFFENSIVE_STRING: &str = "<p>(And if you choose to ignore this warning, you lose all rights to complain about it in the future.)</p>";
        const PINING_STRING: &str = "<p>This gallery is pining for the fjords.</p>";

        if document.contains(OFFENSIVE_STRING) {
            return Err(String::from("And if you choose to ignore this warning, you lose all rights to complain about it in the future."));
        }

        if document.contains(PINING_STRING) {
            return Err(String::from("This gallery is pining for the fjords."));
        }

        // Error info.
        const PATTERN_ERROR: &str = "<div class=\"d\">\n<p>([^<]+)</p>";
        let regex = Regex::new(PATTERN_ERROR).unwrap();
        if let Some(cap) = regex.captures(document) {
            return Err(String::from(&cap[1]));
        }


        todo!();
    }
}

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

        Err(String::from("Parse gallery tag group fail."))
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
const PATTERN_DETAIL: &str = r#"var gid = (\d+;\s*?(\n|\r|\r\n)?\s*?var token = \""([a-f0-9]+)\"";\s*?(\n|\r|\r\n)?\s*?var apiuid = ([\-\d]+;\s*?(\n|\r|\r\n)?\s*?var apikey = \""([a-f0-9]+)\"";"#;
const PATTERN_TORRENT: &str = r#"<a[^<>]*onclick="return popUp\('([^']+)'[^)]+\)">Torrent Download[^<]+(\d+)[^<]+</a"#;
const PATTERN_ARCHIVE: &str = r#"<a[^<>]*onclick="return popUp\('([^']+)'[^)]+\)">Archive Download</a>"#;
const PATTERN_COVER: &str = r#"width:(\d+)px; height:(\d+)px.+?url\((.+?)\)"#;
const PATTERN_TAG_GROUP: &str = r#"<tr><td[^<>]+>([\w\s]+):</td><td>(?:<div[^<>]+><a[^<>]+>[\w\s]+</a></div>)+</td></tr>"#;
const PATTERN_TAG: &str = r#"<div[^<>]+><a[^<>]+>([\w\s]+)</a></div>"#;
const PATTERN_COMMENT: &str = r#"<div class="c3">Posted on ([^<>]+) by: &nbsp; <a[^<>]+>([^<>]+)</a>.+?<div class="c6"[^>]*>(.+?)</div><div class="c[78]""#;
const PATTERN_PAGES: &str = r#"<tr><td[^<>]*>Length:</td><td[^<>]*>([\d,]+) pages</td></tr>"#;
const PATTERN_PREVIEW_PAGES: &str = r#"<td[^>]+><a[^>]+>([\d,]+)</a></td><td[^>]+>(?:<a[^>]+>)?&gt;(?:</a>)?</td>"#;
const PATTERN_NORMAL_PREVIEW: &str = r#"<div class="gdtm"[^<>]*><div[^<>]*width:(\d+)[^<>]*height:(\d+)[^<>]*\((.+?)\)[^<>]*-(\d+)px[^<>]*><a[^<>]*href="(.+?)"[^<>]*><img alt="([\d,]+)""#;
const PATTERN_LARGE_PREVIEW: &str = r#"<div class="gdtl".+?<a href="(.+?)"><img alt="([\d,]+)".+?src="(.+?)""#;
const PATTERN_NEWER_DATE: &str = ", added (.+?)<br />";

#[cfg(test)]
mod tests {
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
}
