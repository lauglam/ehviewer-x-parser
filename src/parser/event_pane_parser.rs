use visdom::Vis;

#[deprecated]
pub fn parse(document: &str) -> Result<String, String> {
    if let Ok(root) = Vis::load(document) {
        let event = root.find("#eventpane");
        Ok(event.outer_htmls())
    } else {
        Err(String::from("Parses event pane fail."))
    }
}
