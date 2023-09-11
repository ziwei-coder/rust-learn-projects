use std::error::Error as StdError;

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;
