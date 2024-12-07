use std::collections::HashSet;

use super::Solution;
use crate::utils::grid_utils::{GridUtils, CARDINAL_OFFSETS};

#[derive(Debug)]
enum Cell {
    Obstacle,
    Empty,
}

#[derive(Debug, Default)]
pub struct D6 {
    guard_position: (i32, i32),
    grid: Vec<Vec<Cell>>,
    visited_in_p1: Vec<Vec<bool>>,
}

impl Solution for D6 {
    fn prepare(&mut self, input: String) {
        let mut guard_position = (0, 0);
        for (i, row) in input.lines().enumerate() {
            self.grid.push(
                row.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '#' => Cell::Obstacle,
                        '^' => {
                            guard_position = (i as i32, j as i32);
                            Cell::Empty
                        }
                        _ => Cell::Empty,
                    })
                    .collect(),
            )
        }
        self.guard_position = guard_position;
    }

    fn p1(&mut self) -> String {
        let mut visited = vec![vec![false; self.grid.width()]; self.grid.height()];
        let mut count = 0;
        let mut offsets = CARDINAL_OFFSETS.iter().cycle();
        let (mut di, mut dj) = offsets.next().unwrap();
        let (mut i, mut j) = self.guard_position;
        'outer: while self.grid.in_bounds(i, j) {
            if !visited[i as usize][j as usize] {
                count += 1;
            }
            visited[i as usize][j as usize] = true;
            loop {
                i += di;
                j += dj;
                if !self.grid.in_bounds(i, j) {
                    break 'outer;
                }

                if let Cell::Obstacle = self.grid[i as usize][j as usize] {
                    i -= di;
                    j -= dj;
                    (di, dj) = *offsets.next().unwrap();
                    continue;
                }
                break;
            }
        }
        self.visited_in_p1 = visited;
        count.to_string()
    }
    fn p2(&mut self) -> String {
        let mut visited: HashSet<(usize, i32, i32)> = HashSet::new();
        let mut count = 0;
        for row in 0..self.grid.height() {
            for col in 0..self.grid.width() {
                visited.clear();
                if !self.visited_in_p1[row][col] || (row as i32, col as i32) == self.guard_position
                {
                    continue;
                }
                self.grid[row][col] = Cell::Obstacle;
                let mut offsets = CARDINAL_OFFSETS.iter().enumerate().cycle();
                let (mut offset_i, (mut di, mut dj)) = offsets.next().unwrap();
                let (mut i, mut j) = self.guard_position;
                'outer: while self.grid.in_bounds(i, j) {
                    if visited.contains(&(offset_i, i, j)) {
                        count += 1;
                        break;
                    }
                    visited.insert((offset_i, i, j));
                    loop {
                        i += di;
                        j += dj;
                        if !self.grid.in_bounds(i, j) {
                            break 'outer;
                        }

                        if let Cell::Obstacle = self.grid[i as usize][j as usize] {
                            i -= di;
                            j -= dj;
                            let (next_oi, next_o) = offsets.next().unwrap();
                            offset_i = next_oi;
                            (di, dj) = *next_o;
                            continue;
                        }
                        break;
                    }
                }

                self.grid[row][col] = Cell::Empty;
            }
        }
        count.to_string()
    }
}
