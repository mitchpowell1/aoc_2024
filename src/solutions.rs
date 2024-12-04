pub mod d1;
pub mod d2;
pub mod d3;
pub mod d4;

pub use d1::D1;
pub use d2::D2;
pub use d3::D3;
pub use d4::D4;

pub trait Solution {
    fn prepare(&mut self, input: String);
    fn p1(&mut self) -> String;
    fn p2(&mut self) -> String;
}
