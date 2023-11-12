use crate::Show;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Open,
    Filled,
    Expired,
}

impl Show for Status {
    fn show(&self) -> String {
        format!("Status {}", serde_json::to_string(&self).unwrap())
    }
}
