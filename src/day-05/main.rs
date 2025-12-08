use std::{collections::HashSet, ops::RangeInclusive};

const INPUT: &str = include_str!("input.txt");

fn main() {
    dbg!(part1());
    dbg!(part2());
}

fn part1() -> u32 {
    let mut sum = 0;
    let (ranges, ids) = parse_input();

    for id in ids.lines() {
        let id: u64 = id.parse().unwrap();

        if ranges.iter().any(|range| range.contains(&id)) {
            sum += 1;
        }
    }

    sum
}

fn part2() -> u64 {
    let mut sum = 0;
    let ranges = parse_input().0;
    let mut removed = HashSet::with_capacity(ranges.len() / 2);

    for (i, range) in ranges.iter().enumerate() {
        if removed.contains(&i) {
            continue;
        }

        let mut start = *range.start();
        let mut end = *range.end();

        while let Some(pos) = ranges
            .iter()
            .enumerate()
            .position(|(j, it)| start <= *it.end() && end >= *it.start() && !removed.contains(&j))
        {
            let overlapped_range = &ranges[pos];
            start = start.min(*overlapped_range.start());
            end = end.max(*overlapped_range.end());
            removed.insert(pos);
        }

        sum += 1 + end - start;
    }

    sum
}

fn parse_input() -> (Vec<RangeInclusive<u64>>, &'static str) {
    let mut ranges = Vec::with_capacity(190);
    let (ranges_str, ids_str) = INPUT.trim_end().split_once("\n\n").unwrap();

    for range_str in ranges_str.lines() {
        let (start, end): (u64, u64) = range_str
            .split_once('-')
            .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
            .unwrap();

        ranges.push(start..=end);
    }

    (ranges, ids_str)
}
