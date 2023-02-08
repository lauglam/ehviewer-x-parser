use visdom::types::Elements;

pub enum Inline {
    MinimalOrMinimalPlus,
    Compact,
    Extended,
    Thumbnail,
}

impl Inline {
    pub fn parse(ele: &Elements) -> Result<Inline, String> {
        if ele.has_class(r#"gltm"#) {
            // Minimal or Minimal+.
            Ok(Inline::MinimalOrMinimalPlus)
        } else if ele.has_class(r#"gltc"#) {
            // Compact.
            Ok(Inline::Compact)
        } else if ele.has_class(r#"glte"#) {
            // Extended.
            Ok(Inline::Extended)
        } else if ele.has_class("gld") {
            // Thumbnail.
            Ok(Inline::Thumbnail)
        } else {
            Err(String::from("parses inline fail."))
        }
    }
}
