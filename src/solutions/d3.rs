use super::Solution;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i32,
    error::{make_error, ErrorKind},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Multiply { a: i32, b: i32 },
}

fn try_parse_do_instruction(input: &str) -> IResult<&str, Instruction> {
    let (parsed, _) = tag("do()")(input)?;
    Ok((parsed, Instruction::Do))
}

fn try_parse_dont_instruction(input: &str) -> IResult<&str, Instruction> {
    let (parsed, _) = tag("don't()")(input)?;
    Ok((parsed, Instruction::Dont))
}

fn try_parse_mult_instruction(input: &str) -> IResult<&str, Instruction> {
    let (parsed, (a, b)) =
        delimited(tag("mul("), separated_pair(i32, tag(","), i32), tag(")"))(input)?;
    let acceptable_range = 0..1000;
    if !acceptable_range.contains(&a) || !acceptable_range.contains(&b) {
        let error = make_error(input, ErrorKind::Fail);
        return Err(nom::Err::Error(error));
    }
    Ok((parsed, Instruction::Multiply { a, b }))
}

fn try_parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        try_parse_do_instruction,
        try_parse_dont_instruction,
        try_parse_mult_instruction,
    ))(input)
}

#[derive(Default)]
pub struct D3 {
    instructions: Vec<Instruction>,
}

impl Solution for D3 {
    fn prepare(&mut self, input: String) {
        let mut offset = 0;
        while offset < input.len() {
            let Ok((parsed, instruction)) = try_parse_instruction(&input[offset..]) else {
                offset += 1;
                continue;
            };

            self.instructions.push(instruction);
            offset = input.len() - parsed.len();
        }
    }

    fn p1(&mut self) -> String {
        self.instructions
            .iter()
            .filter_map(|i| {
                let Instruction::Multiply { a, b } = i else {
                    return None;
                };
                Some(a * b)
            })
            .sum::<i32>()
            .to_string()
    }

    fn p2(&mut self) -> String {
        let mut enabled = true;
        let mut total = 0;
        for i in &self.instructions {
            match i {
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
                Instruction::Multiply { a, b } => {
                    if enabled {
                        total += a * b
                    }
                }
            }
        }

        total.to_string()
    }
}
