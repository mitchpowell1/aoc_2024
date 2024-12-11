use super::Solution;

use fxhash::FxHashMap;

const P1_STEPS: i32 = 25;
const P2_STEPS: i32 = 75;

#[derive(Debug, Clone)]
struct Stone {
    value: u64,
    next: Option<usize>,
}

#[derive(Default, Debug)]
pub struct D11 {
    stones: Vec<Stone>,
}

impl D11 {
    fn get_count_after_steps(&self, steps: i32) -> u64 {
        let mut counts = FxHashMap::default();
        for stone in &self.stones {
            *counts.entry(stone.value).or_default() += 1;
        }
        let mut scratch = FxHashMap::default();

        for _ in 0..steps {
            scratch.clear();
            for (v, count) in &counts {
                match v {
                    // Value of zero
                    0 => *scratch.entry(1).or_default() += count,
                    // Even number of digits
                    v if (v.ilog10() + 1) % 2 == 0 => {
                        let tens = v.ilog10();
                        *scratch.entry(v / (10u64.pow((tens + 1) / 2))).or_default() += count;
                        *scratch.entry(v % (10u64.pow((tens + 1) / 2))).or_default() += count;
                    }
                    // Other
                    _ => *scratch.entry(v * 2024).or_default() += count,
                }
            }
            std::mem::swap(&mut counts, &mut scratch);
        }

        counts.values().sum::<u64>()
    }
}

impl Solution for D11 {
    fn prepare(&mut self, input: String) {
        for (i, n) in input.split_whitespace().enumerate() {
            self.stones.push(Stone {
                value: n.parse().unwrap(),
                next: Some(i + 1),
            })
        }
        if let Some(stone) = self.stones.last_mut() {
            stone.next.take();
        }
    }

    fn p1(&mut self) -> String {
        self.get_count_after_steps(P1_STEPS).to_string()
    }

    fn p2(&mut self) -> String {
        self.get_count_after_steps(P2_STEPS).to_string()
    }
}
