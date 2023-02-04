use visdom::Vis;
use crate::gallery_info::GalleryInfo;
use crate::utils::trim;

#[derive(Debug, PartialEq)]
pub struct Favorite {
    // Size 10
    pub cat_array: [String; 10],
    // Size 10
    pub count_array: [usize; 10],
    pub pages: usize,
    pub next_page: usize,
    pub gallery_info_vec: Vec<GalleryInfo>,
}

pub fn parse(document: &str) -> Result<Favorite, String> {
    if document.contains("This page requires you to log on.</p>") {
        Err(String::from("This page requires you to log on."))
    } else {
        let mut cat_vec = Vec::new();
        let mut count_vec = Vec::new();

        if let Ok(root) = Vis::load(document) {
            let fps = root.find(".ido .fp");

            assert_eq!(10, fps.length());
            for fp in fps {
                let count = fp.child_nodes_item(0).unwrap();
                let cat = fp.child_nodes_item(2).unwrap();
                count_vec.push(count.text().parse::<usize>().unwrap());
                cat_vec.push(trim(&cat.text()).into_owned());
            }


            // let result = gallery_list_parser::

            todo!()
        } else {
            Err(String::from("Parses favorites fail."))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test::read_test_file;
    use super::*;

    #[test]
    fn sign_in_required_test() {
        let document = read_test_file("sign_in_required");
        let result = parse(&document);
        assert_eq!(Err(String::from("This page requires you to log on.")), result)
    }
}
