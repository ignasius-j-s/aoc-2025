const INPUT: &str = include_str!("input.txt");
const ROLL_OF_PAPER: u8 = 0x40; // @ in ascii
const REMOVED: u8 = 0x58; // X in ascii

fn main() {
    dbg!(part1());
    dbg!(part2());
}

fn part1() -> u32 {
    let mut sum = 0;
    let mut lines = INPUT.trim_end().lines().peekable();

    let mut maybe_prev_line: Option<&'static [u8]> = None;

    while let Some(line) = lines.next().map(str::as_bytes) {
        let maybe_next_line = lines.peek().map(|it| it.as_bytes());

        for i in 0..line.len() {
            if line[i] != ROLL_OF_PAPER {
                continue;
            }

            let mut adjacent_roll_count: u8 = 0;

            if let Some(prev_line) = maybe_prev_line {
                adjacent_roll_count += count_neighbor(prev_line, i, false);
            }

            adjacent_roll_count += count_neighbor(line, i, true);

            if let Some(next_line) = maybe_next_line {
                adjacent_roll_count += count_neighbor(next_line, i, false);
            }

            if adjacent_roll_count < 4 {
                sum += 1;
            }
        }

        maybe_prev_line = Some(line);
    }

    sum
}

fn part2() -> u32 {
    let mut sum = 0;
    let mut input = parse_input();

    let mut removed = u32::MAX;
    while removed > 0 {
        removed = 0;

        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] != ROLL_OF_PAPER {
                    continue;
                }

                let mut adjacent_roll_count: u8 = 0;

                if y > 0 {
                    adjacent_roll_count += count_neighbor(input[y - 1].as_slice(), x, false);
                }

                adjacent_roll_count += count_neighbor(input[y].as_slice(), x, true);

                if y < input.len() - 1 {
                    adjacent_roll_count += count_neighbor(input[y + 1].as_slice(), x, false);
                }

                if adjacent_roll_count < 4 {
                    input[y][x] = REMOVED;
                    removed += 1;
                }
            }
        }

        sum += removed;
    }

    sum
}

fn count_neighbor(row: &[u8], index: usize, center: bool) -> u8 {
    let mut count = 0;

    if index > 0 && row[index - 1] == ROLL_OF_PAPER {
        count += 1;
    }

    if !center && row[index] == ROLL_OF_PAPER {
        count += 1;
    }

    if index < row.len() - 1 && row[index + 1] == ROLL_OF_PAPER {
        count += 1;
    }

    count
}

fn parse_input() -> Vec<Vec<u8>> {
    let mut parsed = Vec::with_capacity(140);

    for line in INPUT.trim_end().lines() {
        parsed.push(line.as_bytes().to_owned());
    }

    parsed
}
