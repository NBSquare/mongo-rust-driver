#[cfg(test)]
mod test;

use bson::Document;

use crate::{
    bson::doc,
    cmap::{Command, StreamDescription},
    error::{Error, Result},
    operation::{append_options, Operation, WriteConcernOnlyBody},
    options::{DropCollectionOptions, WriteConcern},
    Namespace,
};

use super::CommandResponse;

#[derive(Debug)]
pub(crate) struct DropCollection {
    ns: Namespace,
    options: Option<DropCollectionOptions>,
}

impl DropCollection {
    pub(crate) fn new(ns: Namespace, options: Option<DropCollectionOptions>) -> Self {
        DropCollection { ns, options }
    }

    #[cfg(test)]
    fn empty() -> Self {
        Self::new(
            Namespace {
                db: String::new(),
                coll: String::new(),
            },
            None,
        )
    }
}

impl Operation for DropCollection {
    type O = ();
    type Command = Document;
    type Response = CommandResponse<WriteConcernOnlyBody>;

    const NAME: &'static str = "drop";

    fn build(&mut self, _description: &StreamDescription) -> Result<Command> {
        let mut body = doc! {
            Self::NAME: self.ns.coll.clone(),
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
        response: WriteConcernOnlyBody,
        _description: &StreamDescription,
    ) -> Result<Self::O> {
        response.validate()
    }

    fn handle_error(&self, error: Error) -> Result<Self::O> {
        if error.is_ns_not_found() {
            Ok(())
        } else {
            Err(error)
        }
    }

    fn write_concern(&self) -> Option<&WriteConcern> {
        self.options
            .as_ref()
            .and_then(|opts| opts.write_concern.as_ref())
    }
}
