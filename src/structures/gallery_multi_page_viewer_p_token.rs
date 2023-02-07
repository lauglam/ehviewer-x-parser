use crate::utils::input::Input;

#[derive(Debug, PartialEq)]
pub struct GalleryMultiPageViewerPToken {
    image_vec: Vec<String>,
}

impl GalleryMultiPageViewerPToken {
    pub fn parse(doc: &str) -> Result<GalleryMultiPageViewerPToken, String> {
        let prefix = r#"var imagelist = "#;

        let mut input = Input::new(doc);
        let bgn = input.find_str(prefix).unwrap();
        input.set_cursor(bgn);
        let end = input.find(';').unwrap();

        let bgn = bgn + prefix.len() + 16;
        let image_list = input.get_string(bgn, end)?;
        if let Ok(image_vec) = serde_json::from_str::<Vec<String>>(&image_list) {
            Ok(GalleryMultiPageViewerPToken {
                image_vec,
            })
        } else {
            Err(String::from("parses gallery multi page viewer ptoken fail."))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {}
}
