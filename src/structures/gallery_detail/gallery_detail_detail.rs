use regex::Regex;
use visdom::Vis;
use crate::{DOM_NOT_FOUND, EhResult, Parser};

#[derive(Debug, PartialEq)]
pub struct GalleryDetailDetail {
    pub posted: String,
    pub parent_opt: Option<String>,
    pub visible: String,
    pub language: String,
    pub file_size: String,
    pub pages: u32,
    pub favorite_count: u32,
}

impl Parser for GalleryDetailDetail {
    /// <table>
    ///     <tr>
    ///         <td class="gdt1">Posted:</td>
    ///         <td class="gdt2">2023-02-07 07:33</td>
    ///     </tr>
    ///     ...
    /// </table>
    fn parse(doc: &str) -> EhResult<Self> {
        const PATTERN_PAGES: &str = r#"(\d+) pages"#;
        const PATTERN_FAVORITE_COUNT: &str = r#"(\d+) times"#;

        let root = Vis::load(doc)?;
        let gdt1s = root.find(".gdt1");

        let (
            mut posted,
            mut parent_opt,
            mut visible,
            mut language,
            mut file_size,
            mut pages,
            mut favorite_count
        ) = (None, None, None, None, None, None, None);

        for gdt1 in gdt1s {
            match gdt1.text().as_str() {
                "Posted:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();
                    posted = Some(gdt2.text());
                }
                "Parent:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();

                    if let Some(href) = gdt2.get_attribute("href") {
                        parent_opt = Some(href.to_string());
                    }
                }
                "Visible:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();
                    visible = Some(gdt2.text());
                }
                "Language:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();
                    language = Some(gdt2.text());
                }
                "File Size:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();
                    file_size = Some(gdt2.text());
                }
                "Length:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();
                    let gdt2 = gdt2.text();

                    let regex = Regex::new(PATTERN_PAGES).unwrap();
                    let captures = regex.captures(&gdt2).unwrap();
                    pages = Some(captures[1].parse()?);
                }
                "Favorited:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();
                    let gdt2 = gdt2.text();

                    let regex = Regex::new(PATTERN_FAVORITE_COUNT).unwrap();
                    let captures = regex.captures(&gdt2).unwrap();
                    favorite_count = Some(captures[1].parse()?);
                }
                _ => unreachable!()
            }
        }

        if let (
            Some(posted),
            Some(visible),
            Some(language),
            Some(file_size),
            Some(pages),
            Some(favorite_count)
        ) = (posted, visible, language, file_size, pages, favorite_count) {
            Ok(GalleryDetailDetail {
                posted,
                parent_opt,
                visible,
                language,
                file_size,
                pages,
                favorite_count,
            })
        } else {
            Err(DOM_NOT_FOUND)
        }
    }
}
