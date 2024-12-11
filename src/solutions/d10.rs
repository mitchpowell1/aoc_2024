use fxhash::FxHashSet;
use itertools::iproduct;

use crate::utils::grid_utils::GridUtils;
use crate::utils::grid_utils::CARDINAL_OFFSETS;

use super::Solution;
use std::collections::VecDeque;

#[derive(Default)]
pub struct D10 {
    grid: Vec<Vec<u8>>,
    visited: FxHashSet<(usize, usize)>,
    to_visit: VecDeque<(usize, usize)>,
}

impl D10 {
    fn get_trailhead_score(&mut self, r: usize, c: usize, check_for_visitation: bool) -> u32 {
        self.visited.clear();
        self.to_visit.clear();

        let mut score = 0;

        self.visited.insert((r, c));
        self.to_visit.push_back((r, c));

        while let Some((r, c)) = self.to_visit.pop_front() {
            let value = self.grid[r][c];
            if value == 9 {
                score += 1;
            }
            for (dr, dc) in CARDINAL_OFFSETS {
                if !self.grid.in_bounds(r as i32 + dr, c as i32 + dc) {
                    continue;
                }
                let r = (r as i32 + *dr) as usize;
                let c = (c as i32 + *dc) as usize;
                if (!check_for_visitation || !self.visited.contains(&(r, c)))
                    && self.grid[r][c] == value + 1
                {
                    self.to_visit.push_back((r, c));
                    self.visited.insert((r, c));
                }
            }
        }

        score
    }
}

impl Solution for D10 {
    fn prepare(&mut self, input: String) {
        for line in input.lines() {
            self.grid.push(line.bytes().map(|b| b - b'0').collect());
        }
    }

    fn p1(&mut self) -> String {
        iproduct!(0..self.grid.height(), 0..self.grid.width())
            .filter_map(|(r, c)| {
                if self.grid[r][c] != 0 {
                    None
                } else {
                    Some(self.get_trailhead_score(r, c, true))
                }
            })
            .sum::<u32>()
            .to_string()
    }

    fn p2(&mut self) -> String {
        iproduct!(0..self.grid.height(), 0..self.grid.width())
            .filter_map(|(r, c)| {
                if self.grid[r][c] != 0 {
                    None
                } else {
                    Some(self.get_trailhead_score(r, c, false))
                }
            })
            .sum::<u32>()
            .to_string()
    }
}
