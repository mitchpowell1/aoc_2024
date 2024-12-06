use std::collections::HashMap;

use super::Solution;

#[derive(Debug, Default)]
pub struct D5 {
    followers: HashMap<i32, Vec<i32>>,
    good_updates: Vec<Vec<i32>>,
    bad_updates: Vec<Vec<i32>>,
}

impl Solution for D5 {
    fn prepare(&mut self, input: String) {
        let mut input_parts = input.split("\n\n");
        for raw_rule in input_parts.next().unwrap().lines() {
            let mut parts = raw_rule.split('|');
            let first = parts.next().unwrap().parse().unwrap();
            let second = parts.next().unwrap().parse().unwrap();
            self.followers.entry(first).or_default().push(second);
        }

        let mut encountered = vec![];
        for raw_update in input_parts.next().unwrap().lines() {
            let mut is_good = true;
            encountered.clear();

            let update = raw_update
                .split(',')
                .filter_map(|v| v.parse().ok())
                .collect();

            for val in &update {
                if let Some(fs) = self.followers.get(val) {
                    for f in fs {
                        if encountered.contains(f) {
                            is_good = false;
                        }
                    }
                }
                encountered.push(*val);
            }

            if is_good {
                self.good_updates.push(update);
            } else {
                self.bad_updates.push(update);
            }
        }
    }

    fn p1(&mut self) -> String {
        self.good_updates
            .iter()
            .map(|update| update[update.len() / 2])
            .sum::<i32>()
            .to_string()
    }

    fn p2(&mut self) -> String {
        let mut scratch_space = vec![];
        self.bad_updates
            .iter_mut()
            .map(|update| {
                scratch_space.clear();
                scratch_space.extend(update.iter().cloned());
                update.sort_by_key(|v| {
                    -(self.followers[v]
                        .iter()
                        .filter(|f| scratch_space.contains(f))
                        .count() as i32)
                });
                update[update.len() / 2]
            })
            .sum::<i32>()
            .to_string()
    }
}
