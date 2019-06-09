use std::result::Result as StdResult;
use std::fmt::Result as FmtResult;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};

pub type EngineResult<T> = StdResult<T, EngineError>;

#[derive(Debug)]
pub struct EngineError {}

impl StdError for EngineError {}

impl Display for EngineError {
	fn fmt(&self, out: &mut Formatter) -> FmtResult {
		match *self {
			_ => write!(out, "Error!")
		}
	}
}