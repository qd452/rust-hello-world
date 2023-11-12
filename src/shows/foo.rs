use crate::Show;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Foo(pub i32);

// #[allow(dead_code)]
impl Show for Foo {
    fn show(&self) -> String {
        format!("Foo {}", serde_json::to_string(&self).unwrap())
    }
}
