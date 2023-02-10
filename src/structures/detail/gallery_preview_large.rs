use regex::Regex;
use visdom::Vis;
use crate::{EhResult, ParseError, Parser, utils::parse_u32};

#[derive(Debug, PartialEq)]
pub struct GalleryPreviewLarge {
    pub position: u32,
    pub filename: String,
    pub page_url: String,
    pub image_url: String,
}

impl Parser for GalleryPreviewLarge {
    /// ```html
    /// <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01"
    ///     title="Page 1: AnMMSC_2_001_1.png"
    ///     src="https://ehgt.org/5b/f9/5bf9580b3b1f63c508a8af85fc73c0567fe93722-12830376-2458-3497-png_l.jpg"></a>     ///
    /// </div>
    /// ```
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;

        let a = root.find("a");
        let href = a.attr("href").ok_or(ParseError::AttributeNotFound("href"))?;
        let page_url = href.to_string();

        let img = a.children("img");
        let src = img.attr("src").ok_or(ParseError::AttributeNotFound("src"))?;
        let image_url = src.to_string();

        let title = img.attr("title").ok_or(ParseError::AttributeNotFound("title"))?;
        let title = title.to_string();
        let regex = Regex::new(PATTERN_FILENAME).unwrap();
        let captures = regex.captures(&title).ok_or(ParseError::RegexMatchFailed)?;
        let filename = String::from(&captures[1]);

        let alt = img.attr("alt").ok_or(ParseError::AttributeNotFound("alt"))?;
        let position = parse_u32(&alt.to_string())? - 1;

        Ok(GalleryPreviewLarge {
            position,
            filename,
            page_url,
            image_url,
        })
    }
}

const PATTERN_FILENAME: &str = r#"Page \d+: ([\w\s]+.[\w]+)"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let ele = r#"
            <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01"
                title="Page 1: AnMMSC_2_001_1.png"
                src="https://ehgt.org/5b/f9/5bf9580b3b1f63c508a8af85fc73c0567fe93722-12830376-2458-3497-png_l.jpg"></a>     ///
            </div>
        "#;

        assert_eq!(GalleryPreviewLarge::parse(ele).is_ok(), true);
    }
}
