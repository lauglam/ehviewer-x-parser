use regex::Regex;
use visdom::{Vis, types::Elements};
use crate::{
    EhResult,
    ParseError,
    Parser,
    ATTRIBUTE_NOT_FOUND,
    structures::gallery_detail::{
        GalleryPreviewLarge,
        GalleryPreviewMedium,
    },
};

#[derive(Debug, PartialEq)]
pub enum GalleryPreviewSet {
    Large(Vec<GalleryPreviewLarge>),
    Medium(Vec<GalleryPreviewMedium>),
}

impl Parser for GalleryPreviewSet {
    /// 1. Medium preview set.
    /// ```html
    /// <div id="gdt">
    ///     <div class="gdtm" style="height:167px">
    ///         <div
    ///             style="margin:1px auto 0; width:100px; height:143px; background:transparent url(https://ehgt.org/m/001496/1496103-00.jpg) -0px 0 no-repeat">
    ///             <a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01" title="Page 1: AnMMSC_2_001_1.png"
    ///                     src="https://ehgt.org/g/blank.gif" style="width:100px; height:142px; margin:-1px 0 0 -1px"></a>
    ///         </div>
    ///     </div>
    ///     <div class="gdtm" style="height:167px">
    ///         <div
    ///             style="margin:1px auto 0; width:100px; height:143px; background:transparent url(https://ehgt.org/m/001496/1496103-00.jpg) -100px 0 no-repeat">
    ///             <a href="https://e-hentai.org/s/ad7a3b7014/1496103-2"><img alt="02" title="Page 2: AnMMSC_2_001_2.jpg"
    ///                     src="https://ehgt.org/g/blank.gif" style="width:100px; height:142px; margin:-1px 0 0 -1px"></a>
    ///         </div>
    ///     </div>
    ///     ...
    ///     <div class="c"></div>
    /// </div>
    /// ```
    /// Or
    /// 2. Large preview set.
    /// ```html
    /// <div id="gdt">
    ///     <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01"
    ///                 title="Page 1: AnMMSC_2_001_1.png"
    ///                 src="https://ehgt.org/5b/f9/5bf9580b3b1f63c508a8af85fc73c0567fe93722-12830376-2458-3497-png_l.jpg"></a>
    ///     </div>
    ///     <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/ad7a3b7014/1496103-2"><img alt="02"
    ///                 title="Page 2: AnMMSC_2_001_2.jpg"
    ///                 src="https://ehgt.org/ad/7a/ad7a3b7014372ce64193c118b1cfcbcf7ea68ee0-2507700-2458-3497-jpg_l.jpg"></a>
    ///     </div>
    ///     ...
    ///     <div class="c"></div>
    /// </div>
    /// ```
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;
        let gdt = root.find("#gdt > div:first-child");
        let kind = gdt.attr("class").ok_or(ATTRIBUTE_NOT_FOUND)?;

        match kind.to_string().as_str() {
            r#"gdtl"# => Ok(GalleryPreviewSet::Large(parse_large(&root)?)),
            r#"gdtm"# => Ok(GalleryPreviewSet::Medium(parse_medium(doc)?)),
            _ => unreachable!(),
        }
    }
}

/// ```html
/// <div id="gdt">
///     <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01"
///                 title="Page 1: AnMMSC_2_001_1.png"
///                 src="https://ehgt.org/5b/f9/5bf9580b3b1f63c508a8af85fc73c0567fe93722-12830376-2458-3497-png_l.jpg"></a>
///     </div>
///     <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/ad7a3b7014/1496103-2"><img alt="02"
///                 title="Page 2: AnMMSC_2_001_2.jpg"
///                 src="https://ehgt.org/ad/7a/ad7a3b7014372ce64193c118b1cfcbcf7ea68ee0-2507700-2458-3497-jpg_l.jpg"></a>
///     </div>
///     ...
///     <div class="c"></div>
/// </div>
/// ```
fn parse_large(root: &Elements) -> Result<Vec<GalleryPreviewLarge>, ParseError> {
    let mut preview_vec = Vec::new();
    let gdt_larges = root.children(r#".gdtl"#);
    for gdt_large in gdt_larges {
        preview_vec.push(GalleryPreviewLarge::parse(&gdt_large.outer_html())?);
    }

    Ok(preview_vec)
}

// const PATTERN_PREVIEW_PAGES: &str = r#"<td[^>]+><a[^>]+>([\d,]+)</a></td><td[^>]+>(?:<a[^>]+>)?&gt;(?:</a>)?</td>"#;
// const PATTERN_LARGE_PREVIEW: &str = r#"<div class="gdtl".+?<a href="(.+?)"><img alt="([\d,]+)".+?src="(.+?)""#;

/// ```html
/// <div id="gdt">
///     <div class="gdtm" style="height:167px">
///         <div
///             style="margin:1px auto 0; width:100px; height:143px; background:transparent url(https://ehgt.org/m/001496/1496103-00.jpg) -0px 0 no-repeat">
///             <a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01" title="Page 1: AnMMSC_2_001_1.png"
///                     src="https://ehgt.org/g/blank.gif" style="width:100px; height:142px; margin:-1px 0 0 -1px"></a>
///         </div>
///     </div>
///     <div class="gdtm" style="height:167px">
///         <div
///             style="margin:1px auto 0; width:100px; height:143px; background:transparent url(https://ehgt.org/m/001496/1496103-00.jpg) -100px 0 no-repeat">
///             <a href="https://e-hentai.org/s/ad7a3b7014/1496103-2"><img alt="02" title="Page 2: AnMMSC_2_001_2.jpg"
///                     src="https://ehgt.org/g/blank.gif" style="width:100px; height:142px; margin:-1px 0 0 -1px"></a>
///         </div>
///     </div>
///     ...
///     <div class="c"></div>
/// </div>
/// ```
fn parse_medium(doc: &str) -> Result<Vec<GalleryPreviewMedium>, ParseError> {
    let mut preview_vec = Vec::new();

    let regex = Regex::new(PATTERN_MEDIUM_PREVIEW).unwrap();
    for cap in regex.captures_iter(doc) {
        let clip_width = cap[1].parse()?;
        let clip_height = cap[2].parse()?;
        let image_url = String::from(&cap[3]);
        let offset_x = cap[4].parse()?;
        let offset_y = 0;
        let page_url = String::from(&cap[5]);
        let position = cap[6].parse::<u32>()? - 1;
        let filename = String::from(&cap[7]);

        preview_vec.push(
            GalleryPreviewMedium {
                position,
                filename,
                page_url,
                image_url,
                offset_x,
                offset_y,
                clip_width,
                clip_height,
            }
        );
    }

    Ok(preview_vec)
}

const PATTERN_MEDIUM_PREVIEW: &str = r#"<div class="gdtm"[^<>]*><div[^<>]*width:(\d+)[^<>]*height:(\d+)[^<>]*\((.+?)\)[^<>]*-(\d+)px[^<>]*><a[^<>]*href="(.+?)"[^<>]*><img alt="([\d,]+)"[^<>]*title="Page \d+: ([\w\s]+.[\w]+)""#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_preview_set_test() {
        let ele = r#"
            <div id="gdt">
                <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01"
                            title="Page 1: AnMMSC_2_001_1.png"
                            src="https://ehgt.org/5b/f9/5bf9580b3b1f63c508a8af85fc73c0567fe93722-12830376-2458-3497-png_l.jpg"></a>
                </div>
                <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/ad7a3b7014/1496103-2"><img alt="02"
                            title="Page 2: AnMMSC_2_001_2.jpg"
                            src="https://ehgt.org/ad/7a/ad7a3b7014372ce64193c118b1cfcbcf7ea68ee0-2507700-2458-3497-jpg_l.jpg"></a>
                </div>
                <div class="c"></div>
            </div>
        "#;

        assert_eq!(GalleryPreviewSet::parse(ele).is_ok(), true);

        let ele = r#"
            <div id="gdt">
                <div class="gdtm" style="height:167px">
                    <div
                        style="margin:1px auto 0; width:100px; height:143px; background:transparent url(https://ehgt.org/m/001496/1496103-00.jpg) -0px 0 no-repeat">
                        <a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01" title="Page 1: AnMMSC_2_001_1.png"
                                src="https://ehgt.org/g/blank.gif" style="width:100px; height:142px; margin:-1px 0 0 -1px"></a>
                    </div>
                </div>
                <div class="gdtm" style="height:167px">
                    <div
                        style="margin:1px auto 0; width:100px; height:143px; background:transparent url(https://ehgt.org/m/001496/1496103-00.jpg) -100px 0 no-repeat">
                        <a href="https://e-hentai.org/s/ad7a3b7014/1496103-2"><img alt="02" title="Page 2: AnMMSC_2_001_2.jpg"
                                src="https://ehgt.org/g/blank.gif" style="width:100px; height:142px; margin:-1px 0 0 -1px"></a>
                    </div>
                </div>
                <div class="c"></div>
            </div>
        "#;

        assert_eq!(GalleryPreviewSet::parse(ele).is_ok(), true);
    }
}
