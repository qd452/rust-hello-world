pub mod closures;
pub mod shows;

pub use shows::{Bar, Foo, Status};

pub trait Show {
    fn show(&self) -> String;
}

pub trait Location {
    fn location(&self) -> String;
}

pub trait ShowLoc: Show + Location {}
