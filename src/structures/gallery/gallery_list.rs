use visdom::Vis;
use crate::structures::{
    gallery::gallery_info::GalleryInfo,
    gallery::inline::Inline,
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

const PATTERN_NEXT_PAGE: &str = r#"page=(\d+)"#;

#[cfg(test)]
mod tests {
    use crate::utils::test::read_test_file;
    use super::*;

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
