use regex::Regex;
use visdom::Vis;
use crate::{ATTRIBUTE_NOT_FOUND, EhResult, Parser, REGEX_MATCH_FAILED};

#[derive(Debug, PartialEq)]
pub struct SearchNav {
    /// First page, value is `None`
    /// 1. Gallery: ?prev=2453492
    /// 2. Favorites: ?prev=1496103-1669783692
    pub prev_opt: Option<String>,
    /// Last page, value is `None`
    /// 1. Gallery: ?next=2453493,
    /// 2. Favorites: ?next=1670171-1669783692
    pub next_opt: Option<String>,
    /// ?next=2453493&jump=1d
    /// ?next=2453493&jump=3d
    /// ?next=2453493&jump=1w
    /// ?next=2453493&jump=2w
    /// ?next=2453493&jump=1m
    /// ?next=2453493&jump=6m
    /// ?next=2453493&jump=1y
    /// ?next=2453493&jump=2y
    pub jump_opt: Option<String>,
    /// ?next=2453493&seek=2023-02-01
    pub seek_opt: Option<String>,
}

impl Parser for SearchNav {
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;
        let u_prev = root.find("#uprev");
        let prev_opt = if let Some(href) = u_prev.attr("href") {
            let href = href.to_string();
            let regex = Regex::new(PATTERN_PREV_PAGE).unwrap();
            let captures = regex.captures(&href).ok_or(REGEX_MATCH_FAILED)?;
            Some(String::from(&captures[1]))
        } else {
            None
        };


        let u_next = root.find("#unext");
        let next_opt = if let Some(href) = u_next.attr("href") {
            let href = href.to_string();
            let regex = Regex::new(PATTERN_NEXT_PAGE).unwrap();
            let captures = regex.captures(&href).ok_or(REGEX_MATCH_FAILED)?;
            Some(String::from(&captures[1]))
        } else {
            None
        };

        let select = root.find("select[onchange]");
        let onchange = select.attr("onchange").ok_or(ATTRIBUTE_NOT_FOUND)?;
        let onchange_str = onchange.to_string();

        let regex = Regex::new(PATTERN_JUMP_PAGE).unwrap();
        let jump_opt = if let Some(cap) = regex.captures(&onchange_str) {
            Some(String::from(&cap[1]))
        } else {
            None
        };

        let regex = Regex::new(PATTERN_SEEK_PAGE).unwrap();
        let seek_opt = if let Some(cap) = regex.captures(&onchange_str) {
            Some(String::from(&cap[1]))
        } else {
            None
        };

        Ok(SearchNav {
            prev_opt,
            next_opt,
            jump_opt,
            seek_opt,
        })
    }
}

const PATTERN_PREV_PAGE: &str = r#"prev=([\d-]+)"#;
const PATTERN_NEXT_PAGE: &str = r#"next=([\d-]+)"#;
const PATTERN_JUMP_PAGE: &str = r#"jump=(\w+)"#;
const PATTERN_SEEK_PAGE: &str = r#"seek=([\w-]+)"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let ele = r#"
            <div class="searchnav">
                <div></div>
                <div><a id="ufirst" href="https://e-hentai.org/">&lt;&lt; First</a></div>
                <div><a id="uprev" href="https://e-hentai.org/?prev=2458771">&lt; Prev</a></div>
                <div id="ujumpbox" class="jumpbox"><a id="ujump" href="javascript:enable_jump_mode('u')">Jump/Seek</a></div>
                <div><a id="unext" href="https://e-hentai.org/?next=2458743">Next &gt;</a></div>
                <div><a id="ulast" href="https://e-hentai.org/?prev=1">Last &gt;&gt;</a></div>
                <div><select onchange="document.location='https://e-hentai.org/?inline_set=dm_'+this.value+'&amp;prev=2458732'">
                        <option value="m" selected="selected">Minimal</option>
                        <option value="p">Minimal+</option>
                        <option value="l">Compact</option>
                        <option value="e">Extended</option>
                        <option value="t">Thumbnail</option>
                    </select></div>
            </div>
        "#;
        assert_eq!(SearchNav::parse(ele).is_ok(), true);

        let jump_ele = r#"
            <div class="searchnav">
                <div></div>
                <div><a id="ufirst" href="https://e-hentai.org/">&lt;&lt; First</a></div>
                <div><a id="uprev" href="https://e-hentai.org/?prev=2458732">&lt; Prev</a></div>
                <div id="ujumpbox" class="jumpbox"><a id="ujump" href="javascript:enable_jump_mode('u')">Jump/Seek</a></div>
                <div><a id="unext" href="https://e-hentai.org/?next=2458691">Next &gt;</a></div>
                <div><a id="ulast" href="https://e-hentai.org/?prev=1">Last &gt;&gt;</a></div>
                <div><select
                        onchange="document.location='https://e-hentai.org/?inline_set=dm_'+this.value+'&amp;next=2458743&amp;jump=1d'">
                        <option value="m" selected="selected">Minimal</option>
                        <option value="p">Minimal+</option>
                        <option value="l">Compact</option>
                        <option value="e">Extended</option>
                        <option value="t">Thumbnail</option>
                    </select></div>
            </div>
        "#;
        let result = SearchNav::parse(jump_ele);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().jump_opt.is_some(), true);

        let seek_ele = r#"
            <div class="searchnav">
                <div></div>
                <div><a id="ufirst" href="https://e-hentai.org/">&lt;&lt; First</a></div>
                <div><a id="uprev" href="https://e-hentai.org/?prev=2458732">&lt; Prev</a></div>
                <div id="ujumpbox" class="jumpbox"><a id="ujump" href="javascript:enable_jump_mode('u')">Jump/Seek</a></div>
                <div><a id="unext" href="https://e-hentai.org/?next=2458691">Next &gt;</a></div>
                <div><a id="ulast" href="https://e-hentai.org/?prev=1">Last &gt;&gt;</a></div>
                <div><select
                        onchange="document.location='https://e-hentai.org/?inline_set=dm_'+this.value+'&amp;next=2464570&amp;seek=2023-02-06'">
                        <option value="m" selected="selected">Minimal</option>
                        <option value="p">Minimal+</option>
                        <option value="l">Compact</option>
                        <option value="e">Extended</option>
                        <option value="t">Thumbnail</option>
                    </select></div>
            </div>
        "#;
        let result = SearchNav::parse(seek_ele);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().seek_opt.is_some(), true);
    }
}
