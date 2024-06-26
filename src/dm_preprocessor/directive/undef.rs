use log::warn;

use crate::{dm_preprocessor::lib::DmPreProcessor, tokens::dm_token::DmToken, util::ParseError};

impl DmPreProcessor {
    pub fn handle_undef(&mut self, args: &[DmToken]) -> Result<(), ParseError> {
        if args.len() != 1 {
            warn!("`undef` requires one argument");
            return Err(ParseError::ERROR_DIRECTIVE_PARSE);
        }
        self.remove_define(args[0].value());
        Ok(())
    }
}
