use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let coords = parse_input();
    let len = coords.len();
    let mut connections = Vec::with_capacity(len * (len - 1) / 2);
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut locations: Vec<Option<usize>> = vec![None; coords.len()];

    for i in 0..len {
        for j in i + 1..len {
            let dist = distance(coords[i], coords[j]);
            connections.push((dist, i, j));
        }
    }

    connections.sort_by_key(|d| d.0);

    let mut part1 = 0;
    let mut part2 = 0;
    let mut connected = 0;

    for &(_, i, j) in connections.iter() {
        if connected == 1000 {
            let mut circuits_len: Vec<usize> = circuits.iter().map(HashSet::len).collect();
            circuits_len.sort();
            part1 = circuits_len.iter().rev().take(3).product();
        }
        connected += 1;

        let circuit_pos = match (locations[i], locations[j]) {
            (None, None) => {
                let pos = circuits.len();
                let mut new_circuit = HashSet::new();

                new_circuit.extend([i, j]);
                circuits.push(new_circuit);
                locations[i] = Some(pos);
                locations[j] = Some(pos);

                pos
            }
            (None, Some(pos)) => {
                circuits[pos].insert(i);
                locations[i] = Some(pos);

                pos
            }
            (Some(pos), None) => {
                circuits[pos].insert(j);
                locations[j] = Some(pos);

                pos
            }
            (Some(p1), Some(p2)) => {
                // already in the same circuit
                if p1 == p2 {
                    continue;
                }

                let (pos1, pos2) = (p1.max(p2), p1.min(p2));

                for i in std::mem::take(&mut circuits[pos2]) {
                    circuits[pos1].insert(i);
                    locations[i] = Some(pos1);
                }

                pos1
            }
        };

        if circuits[circuit_pos].len() == len {
            part2 = coords[i].0 * coords[j].0;
        }
    }

    dbg!(part1, part2);
}

fn parse_input() -> Vec<(u64, u64, u64)> {
    let mut points = Vec::with_capacity(1000);

    for line in INPUT.trim_ascii_end().lines() {
        let mut split = line.split(',');
        let parse = |s: &str| s.parse::<u64>().unwrap();

        let point = (
            split.next().map(parse).unwrap(),
            split.next().map(parse).unwrap(),
            split.next().map(parse).unwrap(),
        );

        points.push(point);
    }

    points
}

fn distance(p1: (u64, u64, u64), q1: (u64, u64, u64)) -> u64 {
    [(p1.0, q1.0), (p1.1, q1.1), (p1.2, q1.2)]
        .iter()
        .map(|&(c1, c2)| c1.abs_diff(c2).pow(2))
        .sum()
}
