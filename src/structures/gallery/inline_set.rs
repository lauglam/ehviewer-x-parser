use visdom::Vis;
use crate::{EhResult, Parser};

pub enum InlineSet {
    Minimal,
    MinimalPlus,
    Compact,
    Extended,
    Thumbnail,
}

impl Parser for InlineSet {
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;
        let selected = root.find("option[selected]");

        Ok(
            match selected.text().as_str() {
                "Minimal" => InlineSet::Minimal,
                "Minimal+" => InlineSet::MinimalPlus,
                "Compact" => InlineSet::Compact,
                "Extended" => InlineSet::Extended,
                "Thumbnail" => InlineSet::Thumbnail,
                _ => unreachable!(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let ele = r#"
            <select onchange="document.location='https://e-hentai.org/?inline_set=dm_'+this.value+''">
                <option value="m" selected="selected">Minimal</option>
                <option value="p">Minimal+</option>
                <option value="l">Compact</option>
                <option value="e">Extended</option>
                <option value="t">Thumbnail</option>
            </select>
        "#;
        assert_eq!(InlineSet::parse(ele).is_ok(), true);

        let ele = r#"
            <select onchange="document.location='https://e-hentai.org/?inline_set=dm_'+this.value+''">
                <option value="m">Minimal</option>
                <option value="p" selected="selected">Minimal+</option>
                <option value="l">Compact</option>
                <option value="e">Extended</option>
                <option value="t">Thumbnail</option>
            </select>
        "#;
        assert_eq!(InlineSet::parse(ele).is_ok(), true);

        let ele = r#"
            <select onchange="document.location='https://e-hentai.org/?inline_set=dm_'+this.value+''">
                <option value="m">Minimal</option>
                <option value="p">Minimal+</option>
                <option value="l" selected="selected">Compact</option>
                <option value="e">Extended</option>
                <option value="t">Thumbnail</option>
            </select>
        "#;
        assert_eq!(InlineSet::parse(ele).is_ok(), true);

        let ele = r#"
            <select onchange="document.location='https://e-hentai.org/?inline_set=dm_'+this.value+''">
                <option value="m">Minimal</option>
                <option value="p">Minimal+</option>
                <option value="l">Compact</option>
                <option value="e" selected="selected">Extended</option>
                <option value="t">Thumbnail</option>
            </select>
        "#;
        assert_eq!(InlineSet::parse(ele).is_ok(), true);

        let ele = r#"
            <select onchange="document.location='https://e-hentai.org/?inline_set=dm_'+this.value+''">
                <option value="m">Minimal</option>
                <option value="p">Minimal+</option>
                <option value="l">Compact</option>
                <option value="e">Extended</option>
                <option value="t" selected="selected">Thumbnail</option>
            </select>
        "#;
        assert_eq!(InlineSet::parse(ele).is_ok(), true);
    }
}
