#![allow(dead_code)]

use regex::Regex;
use visdom::types::Elements;
use visdom::Vis;
use crate::gallery_info::GalleryInfo;
use crate::utils::parse_isize;
use crate::parser::gallery_detail_url_parser::GalleryDetailUrl;

enum Inline {
    MinimalOrMinimalPlus,
    Compact,
    Extended,
    Thumbnail,
}

impl Inline {
    pub fn parse(element: &Elements) -> Inline {
        if element.has_class(r#"gltm"#) {
            // Minimal or Minimal+.
            Inline::MinimalOrMinimalPlus
        } else if element.has_class(r#"gltc"#) {
            // Compact.
            Inline::Compact
        } else if element.has_class(r#"glte"#) {
            // Extended.
            Inline::Extended
        } else {
            // element.has_class("gld")
            // Thumbnail.
            Inline::Thumbnail
        }
    }
}

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
            // todo: parse pages
            let next = 2453493 as usize;
            let jump = None;
            let seek = None;

            // Parses gallery info.
            let itg = root.find(".itg");

            // There are several styles of itg.
            let mut items;
            let inline = Inline::parse(&itg);
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
                gallery_info_vec.push(parse_gallery_info(&item.outer_html(), &inline)?);
            }

            Ok(GalleryList {
                next,
                jump,
                seek,
                gallery_info_vec,
            })
        } else {
            Err(String::from("Parses gallery list fail."))
        }
    }
}

fn parse_gallery_info(element: &str, inline: &Inline) -> Result<GalleryInfo, String> {
    if let Ok(root) = Vis::load(element) {
        // title, gid, token(required), tags
        let url;
        match inline {
            Inline::MinimalOrMinimalPlus | Inline::Compact | Inline::Thumbnail => {
                url = root.find(r#".glname a"#).text();
            }
            Inline::Extended => {
                url = root.find(r#".glname"#).parent("a").text();
            }
        }
        let detail_url = GalleryDetailUrl::parse(&url, true);

        let title = root.find(r#".glink"#).text();

        todo!()
    } else {
        Err(String::from("Parses gallery info fail."))
    }
}

fn parse_pages(document: &str) -> Result<usize, String> {
    todo!()
}

fn parse_rating(rating_style: &str) -> String {
    const PATTERN_RATING: &str = r#"\d+px"#;

    let mut result = String::new();
    let reg = Regex::new(PATTERN_RATING).unwrap();
    let mut n1 = isize::MIN;
    let mut n2 = isize::MIN;

    let mut rate = 5;
    let mut ms = reg.find_iter(rating_style);
    if let Some(m) = ms.next() {
        n1 = parse_isize(&m.as_str().replace("px", ""), 0);
    }

    if let Some(m) = ms.next() {
        n2 = parse_isize(&m.as_str().replace("px", ""), 0);
    }

    if n1 != isize::MIN && n2 != isize::MIN {
        rate -= n1 / 16;
        if n2 == 21 {
            rate -= 1;
            result = rate.to_string();
            result.push_str(".5")
        } else {
            result = rate.to_string();
        }
    }

    result
}

fn parse_favorite_slot(style: &str) -> isize {
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

const PATTERN_THUMB_SIZE: &str = r#"height:(\d+)px;width:(\d+)px"#;
const PATTERN_PAGES: &str = r#"(\d+) page"#;
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
        assert_eq!("4.5", parse_rating(rating_style));
    }

    #[test]
    fn parse_gallery_info_test() {
        // let document = read_test_file("gallery_list_parser_div");
        // let result = GalleryList::parse(&document);

        let document = read_test_file("gallery_list_parser_table");
        let result = GalleryList::parse(&document);
    }
}
