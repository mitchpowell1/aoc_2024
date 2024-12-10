pub mod d1;
pub mod d10;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;
pub mod d7;
pub mod d8;
pub mod d9;

pub use d1::D1;
pub use d10::D10;
pub use d2::D2;
pub use d3::D3;
pub use d4::D4;
pub use d5::D5;
pub use d6::D6;
pub use d7::D7;
pub use d8::D8;
pub use d9::D9;

pub trait Solution {
    fn prepare(&mut self, input: String);
    fn p1(&mut self) -> String;
    fn p2(&mut self) -> String;
}
