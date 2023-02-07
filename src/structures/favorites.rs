use visdom::Vis;
use crate::utils::{parse_u32, trim};

#[derive(Debug, PartialEq)]
pub struct Favorite {
    // Size 10
    pub cat_array: [String; 10],
    // Size 10
    pub count_array: [u32; 10],
    pub pages: u32,
    pub next_page: u32,
    // pub gallery_info_vec: Vec<GalleryInfo>,
}

impl Favorite {
    pub fn parse(doc: &str) -> Result<Self, String> {
        if doc.contains("This page requires you to log on.</p>") {
            return Err(String::from("this page requires you to log on."));
        }

        let mut cat_vec = Vec::new();
        let mut count_vec = Vec::new();

        if let Ok(root) = Vis::load(doc) {
            let fps = root.find(".ido .fp");

            assert_eq!(fps.length(), 10);
            for fp in fps {
                let count = fp.child_nodes_item(0).unwrap();
                let cat = fp.child_nodes_item(2).unwrap();
                count_vec.push(parse_u32(&count.text()));
                cat_vec.push(String::from(trim(&cat.text())));
            }


            // let result = gallery_list_parser::

            todo!()
        } else {
            Err(String::from("parses favorites fail."))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test::read_test_file;
    use super::*;

    #[test]
    fn sign_in_required_test() {
        let doc = read_test_file("sign_in_required.html");
        let result = Favorite::parse(&doc);
        assert_eq!(result, Err(String::from("this page requires you to log on.")));
    }
}
