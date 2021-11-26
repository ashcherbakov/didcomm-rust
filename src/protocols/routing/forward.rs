use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Message;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ParsedForward {
    #[allow(dead_code)]
    pub msg: Message,
    pub next: String,
    pub forwarded_msg: Value,
}
