use super::Solution;
use std::iter::repeat_n;

enum Block {
    File { length: u32, id: i32 },
    Empty { length: u32 },
}

impl Block {
    fn is_empty(&self) -> bool {
        matches!(self, Block::Empty { .. })
    }
}

#[derive(Default)]
pub struct D9 {
    blocks: Vec<Block>,
    // blocks: Vec<Option<i32>>,
}

impl D9 {
    fn print(&self) {
        for b in &self.blocks {
            match b {
                Block::File { length, id } => {
                    for _ in 0..*length {
                        print!("{id}");
                    }
                }
                Block::Empty { length } => {
                    for _ in 0..*length {
                        print!(".");
                    }
                }
            }
        }
        println!();
    }
}

impl Solution for D9 {
    fn prepare(&mut self, input: String) {
        let mut id = 0;
        for (i, digit) in input.trim().chars().enumerate() {
            let length = digit.to_digit(10).unwrap();
            let is_data = i % 2 == 0;

            let block = if i % 2 == 0 {
                Block::File { length, id }
            } else {
                Block::Empty { length }
            };
            self.blocks.push(block);
            // self.blocks.extend(repeat_n(
            //     if is_data { Some(id) } else { None },
            //     digit as usize,
            // ));
            if is_data {
                id += 1;
            }
        }
        self.print();
    }
    fn p1(&mut self) -> String {
        let mut layout = vec![];
        for block in &self.blocks {
            match block {
                Block::File { length, id } => {
                    layout.extend(repeat_n(Some(id), *length as usize));
                }
                Block::Empty { length } => {
                    layout.extend(repeat_n(None, *length as usize));
                }
            }
        }

        let mut head = 0;
        let mut tail = layout.len() - 1;
        loop {
            while head < layout.len() && layout[head].is_some() {
                head += 1;
            }

            while tail > head && layout[tail].is_none() {
                tail -= 1;
            }

            if head >= tail {
                break;
            }

            layout[head] = layout[tail];
            layout[tail] = None;
        }

        let mut out = 0u128;
        for (i, b) in layout.iter().enumerate().filter(|(_, b)| b.is_some()) {
            out += *b.unwrap() as u128 * i as u128;
        }

        for block in layout {
            if let Some(a) = block {
                print!("{a}");
            } else {
                print!(".");
            }
        }
        println!();
        out.to_string()
    }

    fn p2(&mut self) -> String {
        todo!()
    }
}
