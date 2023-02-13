mod gallery_info;

pub use gallery_info::{
    GalleryInfoCompact,
    GalleryInfoExtended,
    GalleryInfoMinimal,
    GalleryInfoMinimalPlus,
    GalleryInfoThumbnail,
};

use visdom::Vis;
use crate::{EhResult, Parser};

#[derive(Debug, PartialEq)]
pub enum GalleryList {
    Minimal(Vec<GalleryInfoMinimal>),
    MinimalPlus(Vec<GalleryInfoMinimalPlus>),
    Compact(Vec<GalleryInfoCompact>),
    Extended(Vec<GalleryInfoExtended>),
    Thumbnail(Vec<GalleryInfoThumbnail>),
}


impl Parser for GalleryList {
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;

        let selector = r#".searchnav select[onchange*=inline_set] > option[selected]"#;
        let selected = root.find(selector).last();

        let itg = root.find(".itg");
        match selected.text().as_str() {
            "Minimal" => {
                let mut vec = Vec::new();
                for child in itg.children("tr").slice(1..) {
                    vec.push(GalleryInfoMinimal::parse(&child.outer_html())?)
                }
                Ok(GalleryList::Minimal(vec))
            }
            "Minimal+" => {
                let mut vec = Vec::new();
                for child in itg.children("tr").slice(1..) {
                    vec.push(GalleryInfoMinimalPlus::parse(&child.outer_html())?)
                }
                Ok(GalleryList::MinimalPlus(vec))
            }
            "Compact" => {
                let mut vec = Vec::new();
                for child in itg.children("tr").slice(1..) {
                    vec.push(GalleryInfoCompact::parse(&child.outer_html())?)
                }
                Ok(GalleryList::Compact(vec))
            }
            "Extended" => {
                let mut vec = Vec::new();
                for child in itg.children("tr") {
                    vec.push(GalleryInfoExtended::parse(&child.outer_html())?)
                }
                Ok(GalleryList::Extended(vec))
            }
            "Thumbnail" => {
                let mut vec = Vec::new();
                for child in itg.children(".gl1t") {
                    vec.push(GalleryInfoThumbnail::parse(&child.outer_html())?)
                }
                Ok(GalleryList::Thumbnail(vec))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::read_test_file;
    use super::*;

    #[test]
    fn parse_gallery_list_test() {
        let doc = read_test_file("gallery_list_minimal.html");
        let result = GalleryList::parse(&doc);

        let doc = read_test_file("gallery_list_minimal_plus.html");
        let result = GalleryList::parse(&doc);

        let doc = read_test_file("gallery_list_compact.html");
        let result = GalleryList::parse(&doc);

        let doc = read_test_file("gallery_list_extended.html");
        let result = GalleryList::parse(&doc);

        let doc = read_test_file("gallery_list_thumbnail.html");
        let result = GalleryList::parse(&doc);
    }
}
