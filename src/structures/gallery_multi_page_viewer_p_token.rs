use crate::{EhResult, Parser, input::Input};

#[derive(Debug, PartialEq)]
pub struct GalleryMultiPageViewerPToken {
    image_vec: Vec<String>,
}

impl Parser for GalleryMultiPageViewerPToken {
    fn parse(doc: &str) -> EhResult<Self> {
        let mut input = Input::new(doc);
        let bgn = input.find_str(PREFIX).unwrap();
        input.set_cursor(bgn);
        let end = input.find(';').unwrap();

        let bgn = bgn + PREFIX.len() + 16;
        let image_list = input.get_string(bgn, end)?;
        let image_vec = serde_json::from_str::<Vec<String>>(&image_list)?;

        Ok(GalleryMultiPageViewerPToken {
            image_vec,
        })
    }
}

const PREFIX: &str = r#"var imagelist = "#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {}
}
