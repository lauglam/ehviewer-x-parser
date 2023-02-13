use visdom::Vis;
use crate::{
    EhResult,
    Parser,
    SIGN_IN_REQUIRED,
    structures::{
        SearchNav,
        gallery_list::GalleryList,
    },
};

#[derive(Debug, PartialEq)]
pub struct Favorite {
    pub search_nav: SearchNav,
    /// Size 10
    pub cat_vec: Vec<String>,
    /// Size 10
    pub count_vec: Vec<u32>,
    pub gallery_list: GalleryList,
}

impl Parser for Favorite {
    fn parse(doc: &str) -> EhResult<Self> {
        if doc.contains("This page requires you to log on.</p>") {
            return Err(SIGN_IN_REQUIRED);
        }

        let mut cat_vec = Vec::new();
        let mut count_vec = Vec::new();

        let root = Vis::load(doc)?;
        // skip last one: <div class="fp fps"...
        let fps = root.find(".ido [class=fp]");

        assert_eq!(fps.length(), 10);
        for fp in fps {
            let children = fp.children();
            let count = children.eq(0);
            let cat = children.eq(2);
            count_vec.push(count.text().parse::<u32>()?);
            cat_vec.push(cat.text());
        }

        let nav = root.find(r#".searchnav"#).eq(0);
        let search_nav = SearchNav::parse(&nav.outer_html())?;
        let gallery_list = GalleryList::parse(doc)?;

        Ok(Favorite {
            search_nav,
            cat_vec,
            count_vec,
            gallery_list,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::read_test_file;
    use super::*;

    #[test]
    fn parse_sign_in_required_test() {
        let doc = read_test_file("sign_in_required.html");
        let result = Favorite::parse(&doc);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn parse_test(){
        let doc = read_test_file("favorites.html");
        assert_eq!(Favorite::parse(&doc).is_ok(),true);
    }
}
