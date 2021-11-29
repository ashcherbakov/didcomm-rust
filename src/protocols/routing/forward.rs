use serde::Serialize;
use serde_json::Value;

use crate::Message;

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct ParsedForward<'a> {
    pub msg: &'a Message,
    pub next: String,
    pub forwarded_msg: Value,
}
