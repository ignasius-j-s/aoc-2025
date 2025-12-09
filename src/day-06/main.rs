const INPUT: &str = include_str!("input.txt");

fn main() {
    dbg!(part1());
    dbg!(part2());
}

fn part1() -> u64 {
    let mut total = 0;
    let mut problems = Vec::new();

    let mut lines = INPUT.trim_end().lines().rev();
    let ops = lines.next().unwrap().split_ascii_whitespace();
    let initial_values = lines.next().unwrap().split_ascii_whitespace();

    for (val, op) in initial_values.zip(ops) {
        problems.push(Problem::new(val, op));
    }

    for line in lines {
        for (val, problem) in line.split_ascii_whitespace().zip(&mut problems) {
            problem.calc(val);
        }
    }

    for problem in problems {
        total += problem.value;
    }

    total
}

fn part2() -> u64 {
    let mut total = 0;

    let mut lines: Vec<&str> = INPUT.lines().collect();
    let ops = lines.pop().unwrap();

    let mut numbers = Vec::new();
    for (i, op) in ops.char_indices().rev() {
        let mut digits = Vec::with_capacity(4);

        for line in lines.iter() {
            let byte = line.as_bytes()[i];

            if byte.is_ascii_digit() {
                digits.push(byte - 0x30);
            }
        }

        if digits.is_empty() {
            continue;
        }

        let mut num = 0;
        for (i, &digit) in digits.iter().rev().enumerate() {
            num += digit as u64 * 10_u64.pow(i as u32);
        }
        numbers.push(num);

        match op {
            '*' => {
                let mut value = numbers.pop().unwrap();
                while let Some(num) = numbers.pop() {
                    value *= num
                }
                total += value;
            }
            '+' => {
                let mut value = 0;
                while let Some(num) = numbers.pop() {
                    value += num
                }
                total += value;
            }
            _ => (),
        }
    }

    total
}

struct Problem {
    value: u64,
    op: Op,
}

impl Problem {
    fn new(value: &str, ops: &str) -> Self {
        let value = value.parse().unwrap();
        let ops = match ops {
            "*" => Op::Mul,
            "+" => Op::Add,
            _ => unreachable!(),
        };

        Self { value, op: ops }
    }

    pub fn calc(&mut self, value: &str) {
        match self.op {
            Op::Add => self.value += value.parse::<u64>().unwrap(),
            Op::Mul => self.value *= value.parse::<u64>().unwrap(),
        }
    }
}

pub enum Op {
    Add,
    Mul,
}
