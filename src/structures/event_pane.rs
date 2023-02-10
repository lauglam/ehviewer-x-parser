use visdom::Vis;
use crate::{EhResult, Parser};

#[deprecated]
#[derive(Debug, PartialEq)]
pub struct EventPane {
    pub value: String,
}

impl Parser for EventPane {
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;

        let event = root.find("#eventpane");
        let value = event.outer_html();

        Ok(EventPane { value })
    }
}
