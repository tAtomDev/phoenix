use std::error::Error;

pub type DynamicError = Box<dyn Error + Send + Sync>;
