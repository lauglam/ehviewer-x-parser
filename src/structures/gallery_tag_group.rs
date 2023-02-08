use visdom::Vis;

#[derive(Debug, PartialEq)]
pub struct GalleryTagGroup {
    pub tag_group_name: String,
    pub tag_vec: Vec<String>,
}

impl ToString for GalleryTagGroup {
    fn to_string(&self) -> String {
        format!("{} ({})", self.tag_group_name, self.size())
    }
}

impl GalleryTagGroup {
    /// ```html
    /// <tr><td class="tc">parody:</td><td><div class="gtl" title="parody:senran kagura">senran kagura</div><div class="gtl" title="parody:the idolmaster">the idolmaster</div></td></tr>
    ///                    ^                                                             ^                                                                 ^
    ///                    tag_group_name                                                tag_vec[0]                                                        tag_vec[1]
    /// ```
    pub fn parse(ele: &str) -> Result<GalleryTagGroup, String> {
        if let Ok(root) = Vis::load(ele) {
            let tag_group_name = root.find(".tc").text();
            let tag_group_name = String::from(&tag_group_name[..tag_group_name.len() - 1]);

            let tag_vec = root.find(".gtl")
                .map(|_, ele| {
                    return ele.text();
                });

            if !tag_group_name.is_empty() && !tag_vec.is_empty() {
                return Ok(GalleryTagGroup {
                    tag_group_name,
                    tag_vec,
                });
            }
        }

        Err(String::from("parses gallery tag group fail."))
    }

    pub fn size(&self) -> usize {
        self.tag_vec.len()
    }

    pub fn get_tag_at(&self, index: usize) -> Option<&String> {
        self.tag_vec.get(index)
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tag_vec.push(tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tag_group_test() {
        let ele = r#"
            <tr>
                <td class="tc">parody:</td>
                <td>
                    <div class="gtl" title="parody:senran kagura">senran kagura</div>
                    <div class="gtl" title="parody:the idolmaster">the idolmaster</div>
                </td>
            </tr>
        "#;

        let tag_group = GalleryTagGroup::parse(&ele).unwrap();
        assert_eq!(tag_group.tag_vec, vec![r#"senran kagura"#, r#"the idolmaster"#]);
        assert_eq!(tag_group.tag_group_name, r#"parody"#);
    }
}
