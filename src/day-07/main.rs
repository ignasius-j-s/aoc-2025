use std::collections::{HashMap, HashSet};

const INPUT: &[u8] = include_bytes!("input.txt");
const SPLITTER: u8 = b'^';
const EMPTY_SPACE: u8 = b'.';

fn main() {
    dbg!(part1());
    dbg!(part2());
}

fn part1() -> u32 {
    let mut sum = 0;
    let mut rows = INPUT.split(|&byte| byte == 0x0A);

    let start = rows
        .next()
        .unwrap()
        .iter()
        .position(|&byte| byte == b'S')
        .unwrap();

    let mut beams = HashSet::from([start]);

    for row in rows {
        if row.is_empty() {
            break;
        }

        let (splitter_count, next_beams) = down1(&beams, row);
        sum += splitter_count;
        beams = next_beams;
    }

    sum
}

fn part2() -> u64 {
    let mut rows = INPUT.split(|&byte| byte == 0x0A);

    let start = rows
        .next()
        .unwrap()
        .iter()
        .position(|&byte| byte == 0x53)
        .unwrap();

    let mut beams = HashMap::new();
    beams.insert(start, 1_u64);

    for row in rows {
        if row.is_empty() {
            break;
        }

        beams = down2(&beams, row);
    }

    beams.values().sum()
}

fn down1(beams: &HashSet<usize>, row: &[u8]) -> (u32, HashSet<usize>) {
    let mut splitter_count = 0;
    let mut next_beams = HashSet::new();

    for pos in beams.iter().copied() {
        match row[pos] {
            EMPTY_SPACE => {
                next_beams.insert(pos);
            }
            SPLITTER => {
                next_beams.insert(pos - 1);
                next_beams.insert(pos + 1);
                splitter_count += 1;
            }
            _ => unreachable!(),
        }
    }

    (splitter_count, next_beams)
}

fn down2(beams: &HashMap<usize, u64>, row: &[u8]) -> HashMap<usize, u64> {
    let mut next_beams = HashMap::new();

    for (&pos, &val) in beams.iter() {
        match row[pos] {
            EMPTY_SPACE => {
                *next_beams.entry(pos).or_default() += val;
            }
            SPLITTER => {
                *next_beams.entry(pos - 1).or_default() += val;
                *next_beams.entry(pos + 1).or_default() += val;
            }
            _ => unreachable!(),
        }
    }

    next_beams
}
