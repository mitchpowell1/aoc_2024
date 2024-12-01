use super::Solution;

use itertools::Itertools;

#[derive(Default)]
pub struct D1 {
    left_list: Vec<i32>,
    right_list: Vec<i32>,
}

impl Solution for D1 {
    fn prepare(&mut self, input: String) {
        for line in input.lines() {
            let mut line_iter = line.split_whitespace();
            self.left_list
                .push(line_iter.next().unwrap().parse().unwrap());
            self.right_list
                .push(line_iter.next().unwrap().parse().unwrap());
        }

        self.left_list.sort();
        self.right_list.sort();
    }

    fn p1(&mut self) -> String {
        self.left_list
            .iter()
            .zip(self.right_list.iter())
            .map(|(l, r)| (l - r).abs())
            .sum::<i32>()
            .to_string()
    }

    fn p2(&mut self) -> String {
        let counts = self.right_list.iter().counts();
        self.left_list
            .iter()
            .map(|v| counts.get(v).cloned().unwrap_or_default() as i32 * v)
            .sum::<i32>()
            .to_string()
    }
}
