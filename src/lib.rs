pub mod closures;
pub mod shows;
pub mod slices;
pub use shows::{Bar, Foo, Status};
pub mod batch_futures;
pub mod common_collections;
pub mod generics;
pub mod lifetime;
pub mod tokio_stuff;

pub trait Show {
    fn show(&self) -> String;
}

pub trait Location {
    fn location(&self) -> String;
}

pub trait ShowLoc: Show + Location {}
