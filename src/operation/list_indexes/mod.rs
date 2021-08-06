use crate::{
    bson::{doc, Document},
    cmap::{Command, StreamDescription},
    cursor::CursorSpecification,
    error::Result,
    operation::{append_options, Operation},
    options::ListIndexOptions,
    IndexModel,
    Namespace,
};

use super::{CursorBody, CursorResponse};

#[cfg(test)]
mod test;

pub(crate) struct ListIndexes {
    ns: Namespace,
    options: Option<ListIndexOptions>,
}

impl ListIndexes {
    pub fn new(ns: Namespace, options: Option<ListIndexOptions>) -> Self {
        ListIndexes { ns, options }
    }
}

impl Operation for ListIndexes {
    type O = CursorSpecification<IndexModel>;
    type Command = Document;
    type Response = CursorResponse<IndexModel>;

    const NAME: &'static str = "listIndexes";

    fn build(&mut self, _description: &StreamDescription) -> Result<Command> {
        let mut body = doc! {
            "listIndexes": self.ns.coll.clone(),
        };
        append_options(&mut body, self.options.as_ref())?;

        Ok(Command::new(
            Self::NAME.to_string(),
            self.ns.db.clone(),
            body,
        ))
    }

    fn handle_response(
        &self,
        response: CursorBody<IndexModel>,
        description: &StreamDescription,
    ) -> Result<Self::O> {
        Ok(CursorSpecification::new(
            response.cursor,
            description.server_address.clone(),
            self.options.as_ref().and_then(|o| o.batch_size),
            self.options.as_ref().and_then(|o| o.max_time),
        ))
    }
}
