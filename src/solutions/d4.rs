use super::Solution;

use crate::utils::grid_utils::{GridUtils, ALL_OFFSETS, DIAGONAL_OFFSETS};

const LETTERS: &[char] = &['X', 'M', 'A', 'S'];
const IN_FORWARD_DIAGONAL: u8 = 1;
const IN_BACKWARD_DIAGONAL: u8 = 2;

#[derive(Default)]
pub struct D4 {
    grid: Vec<Vec<char>>,
}

impl Solution for D4 {
    fn prepare(&mut self, input: String) {
        for row in input.lines() {
            self.grid.push(row.chars().collect())
        }
    }

    fn p1(&mut self) -> String {
        let mut count = 0;
        for i in 0..self.grid.height() {
            for j in 0..self.grid.width() {
                for (dr, dc) in ALL_OFFSETS {
                    let mut r = i as i32;
                    let mut c = j as i32;
                    let mut letters_offset = 0;
                    while self.grid.in_bounds(r, c)
                        && self.grid[r as usize][c as usize] == LETTERS[letters_offset]
                    {
                        letters_offset += 1;
                        if letters_offset == LETTERS.len() {
                            count += 1;
                            break;
                        }
                        r += dr;
                        c += dc;
                    }
                }
            }
        }
        count.to_string()
    }

    fn p2(&mut self) -> String {
        let mut memo = vec![vec![0u8; self.grid.width()]; self.grid.height()];
        let mut count = 0;

        for i in 0..self.grid.height() {
            for j in 0..self.grid.width() {
                for (dr, dc) in DIAGONAL_OFFSETS {
                    let mut r = i as i32;
                    let mut c = j as i32;
                    let mut letters_offset = 1;
                    while self.grid.in_bounds(r, c)
                        && self.grid[r as usize][c as usize] == LETTERS[letters_offset]
                    {
                        letters_offset += 1;
                        if letters_offset == LETTERS.len() {
                            loop {
                                let row = r as usize;
                                let col = c as usize;
                                let should_break = row == i && col == j;
                                memo[row][col] |= if dr == dc {
                                    IN_FORWARD_DIAGONAL
                                } else {
                                    IN_BACKWARD_DIAGONAL
                                };
                                if self.grid[row][col] == 'A'
                                    && memo[row][col] == IN_FORWARD_DIAGONAL | IN_BACKWARD_DIAGONAL
                                {
                                    count += 1
                                }
                                if should_break {
                                    break;
                                }
                                r -= dr;
                                c -= dc;
                            }
                            break;
                        }
                        r += dr;
                        c += dc;
                    }
                }
            }
        }

        count.to_string()
    }
}
