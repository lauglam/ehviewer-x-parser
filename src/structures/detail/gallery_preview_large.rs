use regex::Regex;
use visdom::Vis;
use crate::utils::parse_u32;

#[derive(Debug, PartialEq)]
pub struct GalleryPreviewLarge {
    pub position: u32,
    pub filename: String,
    pub page_url: String,
    pub image_url: String,
}

impl GalleryPreviewLarge {
    /// ```html
    /// <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01"
    ///     title="Page 1: AnMMSC_2_001_1.png"
    ///     src="https://ehgt.org/5b/f9/5bf9580b3b1f63c508a8af85fc73c0567fe93722-12830376-2458-3497-png_l.jpg"></a>     ///
    /// </div>
    /// ```
    pub fn parse(ele: &str) -> Result<GalleryPreviewLarge, String> {
        parse_internal(ele).ok_or(String::from("parses gallery preview set large fail."))
    }
}

pub fn parse_internal(ele: &str) -> Option<GalleryPreviewLarge> {
    const PATTERN_FILENAME: &str = r#"Page \d+: ([\w\s]+.[\w]+)"#;
    let root = Vis::load(ele).ok()?;

    let a = root.find("a");
    let href = a.attr("href")?;
    let page_url = href.to_string();

    let img = a.children("img");
    let src = img.attr("src")?;
    let image_url = src.to_string();

    let title = img.attr("title")?;
    let title = title.to_string();
    let regex = Regex::new(PATTERN_FILENAME).unwrap();
    let captures = regex.captures(&title)?;
    let filename = String::from(&captures[1]);

    let alt = img.attr("alt")?;
    let position = parse_u32(&alt.to_string()).ok()? - 1;

    Some(GalleryPreviewLarge {
        position,
        filename,
        page_url,
        image_url,
    })
}

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
