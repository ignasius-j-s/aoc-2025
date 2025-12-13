const INPUT: &str = include_str!("input.txt");

fn main() {
    let jbox_coords = parse_input();
    let jbox_count = jbox_coords.len();
    let mut connections = Vec::with_capacity(jbox_count * (jbox_count - 1) / 2);
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    let mut circuit_of: Vec<Option<usize>> = vec![None; jbox_count];

    for i in 0..jbox_count {
        for j in i + 1..jbox_count {
            let dist = distance(jbox_coords[i], jbox_coords[j]);
            connections.push((dist, i, j));
        }
    }

    connections.sort_by_key(|d| d.0);

    let mut part1 = 0;
    let mut part2 = 0;

    for (connected, &(_, i, j)) in connections.iter().enumerate() {
        if connected == 1000 {
            let mut biggest_len = [0; 3];

            for len in circuits.iter().map(Vec::len) {
                for i in 0..3 {
                    if len > biggest_len[i] {
                        biggest_len[i..].rotate_right(1);
                        biggest_len[i] = len;
                        break;
                    }
                }
            }

            part1 = biggest_len.iter().product();
        }

        let circuit_pos = match (circuit_of[i], circuit_of[j]) {
            (None, None) => {
                let pos = circuits.len();
                let new_circuit = vec![i, j];

                circuits.push(new_circuit);
                circuit_of[i] = Some(pos);
                circuit_of[j] = Some(pos);

                pos
            }
            (None, Some(pos)) => {
                circuits[pos].push(i);
                circuit_of[i] = Some(pos);

                pos
            }
            (Some(pos), None) => {
                circuits[pos].push(j);
                circuit_of[j] = Some(pos);

                pos
            }
            (Some(p1), Some(p2)) => {
                // already in the same circuit
                if p1 == p2 {
                    continue;
                }

                let (pos1, pos2) = (p1.max(p2), p1.min(p2));

                for i in std::mem::take(&mut circuits[pos2]) {
                    circuits[pos1].push(i);
                    circuit_of[i] = Some(pos1);
                }

                pos1
            }
        };

        if circuits[circuit_pos].len() == jbox_count {
            part2 = jbox_coords[i].0 * jbox_coords[j].0;
            break;
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
