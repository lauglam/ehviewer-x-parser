use visdom::Vis;

#[derive(Debug, PartialEq)]
pub struct GalleryNotAvailable {
    pub error: String,
}

impl GalleryNotAvailable {
    pub fn parse(doc: &str) -> Result<GalleryNotAvailable, String> {
        let root = Vis::load(doc).map_err(|_| String::from("parses gallery not available fail."))?;
        let p = root.find(".d p:first-child");
        let error = p.text();

        Ok(GalleryNotAvailable {
            error,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test::read_test_file;
    use super::*;

    #[test]
    fn parse_test() {
        let doc = read_test_file("gallery_not_available.html");
        assert_eq!(GalleryNotAvailable::parse(&doc).unwrap(), GalleryNotAvailable {
            error: String::from("This gallery has been removed or is unavailable.")
        })
    }
}
