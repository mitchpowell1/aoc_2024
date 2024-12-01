use std::fmt::Debug;

pub mod d1;

pub use d1::D1;

pub trait Solution {
    fn prepare(&mut self, input: String);
    fn p1(&mut self) -> String;
    fn p2(&mut self) -> String;
}
