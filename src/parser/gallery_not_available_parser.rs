use visdom::Vis;

#[derive(Debug, PartialEq)]
pub struct GalleryNotAvailable {
    pub error: String,
}

impl GalleryNotAvailable {
    pub fn parse(document: &str) -> Result<GalleryNotAvailable, String> {
        if let Ok(root) = Vis::load(document) {
            let p = root.find(".d p:first-child");
            let error = p.text();

            Ok(GalleryNotAvailable {
                error,
            })
        } else {
            Err(String::from("Parses gallery not available fail."))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test::read_test_file;
    use super::*;

    #[test]
    fn parse_test() {
        let document = read_test_file("gallery_not_available_parser");
        assert_eq!(GalleryNotAvailable::parse(&document).unwrap(), GalleryNotAvailable {
            error: String::from("This gallery has been removed or is unavailable.")
        })
    }
}
