use super::Solution;

#[derive(Debug)]
struct Equation {
    answer: i64,
    operands: Vec<i64>,
}

#[derive(Debug, Default)]
pub struct D7 {
    equations: Vec<Equation>,
}

fn can_be_solved_p1(ans: i64, target: i64, operands: &[i64]) -> bool {
    if operands.is_empty() {
        return target == ans;
    }

    can_be_solved_p1(ans + operands[0], target, &operands[1..])
        || can_be_solved_p1(ans * operands[0], target, &operands[1..])
}

fn can_be_solved_p2(ans: i64, target: i64, operands: &[i64]) -> bool {
    if operands.is_empty() {
        return target == ans;
    }

    let mut pow = 10;
    while operands[0] >= pow {
        pow *= 10;
        continue;
    }
    let concatenated = ans * pow + operands[0];

    can_be_solved_p2(ans + operands[0], target, &operands[1..])
        || can_be_solved_p2(ans * operands[0], target, &operands[1..])
        || can_be_solved_p2(concatenated, target, &operands[1..])
}

impl Solution for D7 {
    fn prepare(&mut self, input: String) {
        self.equations.extend(input.lines().map(|line| {
            let mut parts = line.split(':');
            let answer = parts.next().unwrap().parse().unwrap();
            let operands = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|o| o.parse().unwrap())
                .collect();

            Equation { answer, operands }
        }));
    }

    fn p1(&mut self) -> String {
        self.equations
            .iter()
            .filter(|eq| can_be_solved_p1(eq.operands[0], eq.answer, &eq.operands[1..]))
            .map(|eq| eq.answer)
            .sum::<i64>()
            .to_string()
    }

    fn p2(&mut self) -> String {
        self.equations
            .iter()
            .filter(|eq| can_be_solved_p2(eq.operands[0], eq.answer, &eq.operands[1..]))
            .map(|eq| eq.answer)
            .sum::<i64>()
            .to_string()
    }
}
