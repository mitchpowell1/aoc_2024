use super::Solution;

#[derive(Default)]
pub struct D2 {
    reports: Vec<Vec<i32>>,
}

fn check_values(vs: &[i32], mut seed: i32, mut can_skip: bool) -> bool {
    let signum = (vs[0] - seed).signum();
    if signum == 0 {
        return false;
    }
    for v2 in vs {
        let diff = v2 - seed;
        if diff.signum() != signum || diff.abs() > 3 {
            if can_skip {
                can_skip = false;
                continue;
            }
            return false;
        }

        seed = *v2
    }

    true
}

impl Solution for D2 {
    fn prepare(&mut self, input: String) {
        for line in input.lines() {
            self.reports.push(
                line.split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            )
        }
    }
    // Safe if...
    // The levels are either all increasing or all decreasing.
    // Any two adjacent levels differ by at least one and at most three.
    fn p1(&mut self) -> String {
        let good_reports = self
            .reports
            .iter()
            .filter(|report| check_values(&report[1..], report[0], false))
            .count();

        format!("{good_reports}")
    }

    // Safe if...
    // You can skip at most one value and the conditions
    // for p1 hold
    fn p2(&mut self) -> String {
        let good_reports = self
            .reports
            .iter()
            .filter(|report| {
                check_values(&report[1..], report[0], true)
                    || check_values(&report[2..], report[0], false)
                    || check_values(&report[2..], report[1], false)
            })
            .count();

        format!("{good_reports}")
    }
}
