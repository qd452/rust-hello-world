pub mod shows;

pub use shows::{Foo, Status};

pub trait Show {
    fn show(&self) -> String;
}
