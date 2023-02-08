#[derive(Debug, PartialEq)]
pub struct GalleryPreviewSet {}


const PATTERN_PREVIEW_PAGES: &str = r#"<td[^>]+><a[^>]+>([\d,]+)</a></td><td[^>]+>(?:<a[^>]+>)?&gt;(?:</a>)?</td>"#;
const PATTERN_NORMAL_PREVIEW: &str = r#"<div class="gdtm"[^<>]*><div[^<>]*width:(\d+)[^<>]*height:(\d+)[^<>]*\((.+?)\)[^<>]*-(\d+)px[^<>]*><a[^<>]*href="(.+?)"[^<>]*><img alt="([\d,]+)""#;
const PATTERN_LARGE_PREVIEW: &str = r#"<div class="gdtl".+?<a href="(.+?)"><img alt="([\d,]+)".+?src="(.+?)""#;
