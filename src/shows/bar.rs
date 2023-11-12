use crate::{Location, Show, ShowLoc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bar {
    name: String,
    loc: String,
}

impl Bar {
    pub fn new(name: &str, location: &str) -> Self {
        Self {
            name: name.to_string(),
            loc: location.to_string(),
        }
    }
}

// #[allow(dead_code)]
/// Implements the `Show` trait for `Bar` struct.
impl Show for Bar {
    /// Returns a formatted string representation of `Bar` struct.
    fn show(&self) -> String {
        format!("Bar<{}>", self.name)
    }
}

impl Location for Bar {
    fn location(&self) -> String {
        self.loc.clone()
    }
}
/// Implements the `ShowLoc` trait for `Bar`.
impl ShowLoc for Bar {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_new() {
        let bar = Bar::new("Test Bar", "Test Location");
        assert_eq!(bar.name, "Test Bar");
        assert_eq!(bar.loc, "Test Location");
    }

    #[test]
    fn test_bar_show() {
        let bar = Bar {
            name: String::from("bar"),
            loc: String::from("Test Location"),
        };
        assert_eq!(bar.show(), "Bar<bar>");
    }

    #[test]
    fn test_bar_location() {
        let bar = Bar {
            name: String::from("bar"),
            loc: String::from("Test Location"),
        };
        assert_eq!(bar.location(), "Test Location");
    }
}
