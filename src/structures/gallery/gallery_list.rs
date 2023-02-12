use visdom::Vis;
use crate::{
    EhResult,
    Parser,
    structures::gallery::{
        GalleryInfo,
        InlineSet,
    },
};

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

impl Parser for GalleryList {
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;

        // TODO parse pages
        let next = 2453493 as usize;
        let jump = None;
        let seek = None;

        let inline_set = root.find(r#".searchnav select[onchange*=inline_set]"#);
        let inline_set = InlineSet::parse(&inline_set.outer_html())?;

        // parses gallery info.
        let itg = root.find(".itg");

        // there are several styles of itg.
        let mut items;
        match inline_set {
            InlineSet::Minimal | InlineSet::MinimalPlus | InlineSet::Compact => {
                // Minimal or Minimal+ or Compact.
                items = itg.children("tr");
                // First one is header, skip it.
                items = items.slice(1..);
            }
            InlineSet::Extended => {
                // Extended.
                items = itg.children("tr");
            }
            InlineSet::Thumbnail => {
                // Thumbnail.
                items = itg.find(".gl1t")
            }
        }

        let mut gallery_info_vec = Vec::new();
        for item in items {
            gallery_info_vec.push(GalleryInfo::parse(&item.outer_html())?);
        }

        Ok(GalleryList {
            next,
            jump,
            seek,
            gallery_info_vec,
        })
    }
}

// const PATTERN_NEXT_PAGE: &str = r#"page=(\d+)"#;

#[cfg(test)]
mod tests {
    use crate::utils::test::read_test_file;
    use super::*;

    #[test]
    fn parse_gallery_info_test() {
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
