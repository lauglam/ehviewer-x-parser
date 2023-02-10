use regex::Regex;
use visdom::Vis;
use std::collections::HashMap;
use crate::{
    EhResult,
    Parser,
    ParseError,
    utils::{
        parse_f32,
        parse_u32,
        parse_u64,
        unescape,
    }, structures::{
        Category,
        FavoriteSlot,
        GalleryTagGroup,
        GalleryDetailUrl,
        GalleryTagGroupList,
        detail::{
            GalleryCommentList,
            GalleryDetailDetail,
            GalleryPreviewSet,
        },
    },
};

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
    pub comment_list: GalleryCommentList,
    pub preview_pages: u32,
    pub preview_set: GalleryPreviewSet,
    pub url: String,
    pub title: String,
    pub title_jpn: String,
    pub category: u32,
    pub uploader: String,
    pub rating_opt: Option<f32>,
    pub detail: GalleryDetailDetail,
}

impl Parser for GalleryDetail {
    fn parse(doc: &str) -> EhResult<Self> {
        if doc.contains(OFFENSIVE_STRING) {
            return Err(ParseError::FromServer(String::from("if you choose to ignore this warning, you lose all rights to complain about it in the future.")));
        }

        if doc.contains(PINING_STRING) {
            return Err(ParseError::FromServer(String::from("this gallery is pining for the fjords.")));
        }

        let regex = Regex::new(PATTERN_ERROR).unwrap();
        if let Some(cap) = regex.captures(doc) {
            return Err(ParseError::FromServer(String::from(&cap[1])));
        }

        let regex = Regex::new(PATTERN_DETAIL).unwrap();
        let captures = regex.captures(doc).ok_or(ParseError::RegexMatchFailed)?;
        let gid = parse_u64(&captures[1])?;
        let api_uid = parse_u64(&captures[5])?;
        let token = String::from(&captures[3]);
        let api_key = String::from(&captures[7]);

        let regex = Regex::new(PATTERN_TORRENT).unwrap();
        let captures = regex.captures(doc).ok_or(ParseError::RegexMatchFailed)?;
        let torrent_url = String::from(unescape(&captures[1]));
        let torrent_count = parse_u32(&captures[2])?;

        let regex = Regex::new(PATTERN_ARCHIVE).unwrap();
        let captures = regex.captures(doc).ok_or(ParseError::RegexMatchFailed)?;
        let archive_url = String::from(unescape(&captures[1]));

        let root = Vis::load(doc)?;
        let gm = root.find(".gm:not(#cdiv)");

        let cover = gm.find("#gd1 div:first-child");
        let style = cover.attr("style").ok_or(ParseError::AttributeNotFound("style"))?;
        let style = style.to_string();
        let regex = Regex::new(PATTERN_COVER).unwrap();
        let captures = regex.captures(&style).ok_or(ParseError::RegexMatchFailed)?;
        let thumb = String::from(&captures[3]);

        let gn = gm.find("#gn");
        let title = gn.text();

        let gj = gm.find("#gj");
        let title_jpn = gj.text();

        let cs = gm.find("#gdc > .cs");
        let cs = cs.text();
        let category = Category::from(&cs).value;

        let gdn = gm.find("#gdn");
        let uploader = gdn.text();

        let gdd = gm.find("#gdd");
        let detail = GalleryDetailDetail::parse(&gdd.html())?;

        let rat = gm.find("#rating_count");
        let rating_count = parse_u32(&rat.text())?;

        let label = gm.find("#rating_label");
        let label_text = label.text();
        let mut rating_opt: Option<f32> = None;
        if label_text != "Not Yet Rated" {
            let regex = Regex::new(PATTERN_RATING).unwrap();
            let captures = regex.captures(&label_text).ok_or(ParseError::RegexMatchFailed)?;
            rating_opt = Some(parse_f32(&captures[1])?);
        }

        let gdf = gm.find("#gdf");
        let favorite_link = gdf.find("#favoritelink");
        let is_favorited = !favorite_link.text().contains("Add to Favorites");

        let (favorite_slot_opt, favorite_name_opt) = if is_favorited {
            let i = gdf.find(".i");
            let style = i.attr("style").ok_or(ParseError::AttributeNotFound("style"))?;
            let favorite_slot = FavoriteSlot::parse(&style.to_string())?.value;

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
                let href = href.get_attribute("href").ok_or(ParseError::AttributeNotFound("href"))?;
                let detail_url = GalleryDetailUrl::parse(&href.to_string())?;
                newer_version_map.insert(date_vec[idx].clone(), detail_url);
            }
            Some(newer_version_map)
        } else {
            None
        };

        let c_div = root.find("#cdiv");
        let comment_list = GalleryCommentList::parse(&c_div.outer_html())?;

        let last_page = root.find(".ptt td:nth-last-child(2) > a");
        let preview_pages = parse_u32(&last_page.text())?;

        let first_page = root.find(".ptt td:nth-child(2) > a");
        let href = first_page.attr("href").ok_or(ParseError::AttributeNotFound("href"))?;
        let url = href.to_string();

        let gdo4 = root.find("#gdo4");
        let can_click = gdo4.children("[onclick]");

        let gdt = root.find("#gdt");
        let preview_set = match can_click.text().as_str() {
            "Large" => GalleryPreviewSet::parse(&gdt.outer_html())?,
            "Normal" => GalleryPreviewSet::parse(&gdt.outer_html())?,
            _ => unreachable!(),
        };

        let tag_list = root.find("#taglist");
        let tag_group_list = GalleryTagGroupList::parse(&tag_list.outer_html())?;
        let tag_group_vec = tag_group_list.group_vec;

        Ok(GalleryDetail {
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
            tag_group_vec,
            comment_list,
            preview_pages,
            preview_set,
            url,
            title,
            title_jpn,
            category,
            uploader,
            rating_opt,
            detail,
        })
    }
}

const OFFENSIVE_STRING: &str = "<p>(And if you choose to ignore this warning, you lose all rights to complain about it in the future.)</p>";
const PINING_STRING: &str = "<p>This gallery is pining for the fjords.</p>";
const PATTERN_ERROR: &str = "<div class=\"d\">\n<p>([^<]+)</p>";
const PATTERN_DETAIL: &str = "var gid = (\\d+);\\s*?(\n|\r|\r\n)?\\s*?var token = \"([a-f0-9]+)\";\\s*?(\n|\r|\r\n)?\\s*?var apiuid = ([\\-\\d]+);\\s*?(\n|\r|\r\n)?\\s*?var apikey = \"([a-f0-9]+)\";";
const PATTERN_TORRENT: &str = r#"<a[^<>]*onclick="return popUp\('([^']+)'[^)]+\)">Torrent Download[^<]+(\d+)[^<]+</a"#;
const PATTERN_ARCHIVE: &str = r#"<a[^<>]*onclick="return popUp\('([^']+)'[^)]+\)">Archive Download</a>"#;
const PATTERN_RATING: &str = r#"[+-]?([0-9]*[.]?[0-9]+)"#;
const PATTERN_NEWER_DATE: &str = ", added (.+?)<br />";
const PATTERN_COVER: &str = r#"width:(\d+)px; height:(\d+)px.+?url\((.+?)\)"#;
// const PATTERN_PAGES: &str = r#"<tr><td[^<>]*>Length:</td><td[^<>]*>([\d,]+) pages</td></tr>"#;

#[cfg(test)]
mod tests {
    use crate::utils::test::read_test_file;
    use super::*;

    #[test]
    fn parse_test() {
        let doc = read_test_file("gallery_detail.html");
        assert_eq!(GalleryDetail::parse(&doc).is_ok(), true);
    }

    // #[test]
    // fn parse_cover_style_test() {
    //     let style = r#"width:250px; height:354px; background:transparent url(https://ehgt.org/8f/3e/8f3ed3234614db3932038b8d7c80a6fd17fe2c41-2942019-2828-4000-jpg_250.jpg) no-repeat"#;
    //     assert_eq!(parse_cover_style(style).unwrap(), r#"https://ehgt.org/8f/3e/8f3ed3234614db3932038b8d7c80a6fd17fe2c41-2942019-2828-4000-jpg_250.jpg"#);
    // }

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
