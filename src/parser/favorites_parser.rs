use visdom::Vis;
// use crate::gallery_info::GalleryInfo;
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

pub fn parse(document: &str) -> Result<Favorite, String> {
    if document.contains("This page requires you to log on.</p>") {
        Err(String::from("this page requires you to log on."))
    } else {
        let mut cat_vec = Vec::new();
        let mut count_vec = Vec::new();

        if let Ok(root) = Vis::load(document) {
            let fps = root.find(".ido .fp");

            assert_eq!(fps.length(), 10);
            for fp in fps {
                let count = fp.child_nodes_item(0).unwrap();
                let cat = fp.child_nodes_item(2).unwrap();
                count_vec.push(parse_u32(&count.text()));
                cat_vec.push(trim(&cat.text()).into_owned());
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
        let document = read_test_file("sign_in_required.html");
        let result = parse(&document);
        assert_eq!(result, Err(String::from("this page requires you to log on.")));
    }
}
