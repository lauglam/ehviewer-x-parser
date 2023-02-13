use visdom::Vis;
use crate::{
    EhResult,
    Parser,
    structures::{
        SearchNav,
        gallery_list::GalleryList,
    },
};

#[derive(Debug, PartialEq)]
pub struct Gallery {
    pub search_nav: SearchNav,
    pub gallery_list: GalleryList,
}

impl Parser for Gallery {
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;
        let nav = root.find(r#".searchnav"#).eq(0);
        let search_nav = SearchNav::parse(&nav.outer_html())?;

        let gallery_list = GalleryList::parse(doc)?;

        Ok(Gallery {
            search_nav,
            gallery_list,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::read_test_file;
    use super::*;

    #[test]
    fn parse_gallery_test() {
        let doc = read_test_file("gallery_list_minimal.html");
        let result = Gallery::parse(&doc);

        let doc = read_test_file("gallery_list_minimal_plus.html");
        let result = Gallery::parse(&doc);

        let doc = read_test_file("gallery_list_compact.html");
        let result = Gallery::parse(&doc);

        let doc = read_test_file("gallery_list_extended.html");
        let result = Gallery::parse(&doc);

        let doc = read_test_file("gallery_list_thumbnail.html");
        let result = Gallery::parse(&doc);
    }
}
