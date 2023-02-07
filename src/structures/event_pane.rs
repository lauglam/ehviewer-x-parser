use visdom::Vis;

#[deprecated]
#[derive(Debug, PartialEq)]
pub struct EventPane {
    pub value: String,
}

impl EventPane {
    pub fn parse(doc: &str) -> Result<Self, String> {
        let root = Vis::load(doc).map_err(|_|String::from("parses event pane fail."))?;

        let event = root.find("#eventpane");
        let value = event.outer_html();
        Ok(EventPane {
            value,
        })
    }
}
