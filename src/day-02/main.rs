const INPUT: &str = include_str!("input.txt");

// i wont be using string

fn main() {
    dbg!(part1());
    dbg!(part2());
}

fn part1() -> u64 {
    let mut sum = 0;
    for id_ranges in INPUT.trim_end().split(',') {
        let (first_id, last_id): (u64, u64) = id_ranges
            .split_once('-')
            .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
            .unwrap();

        for id in first_id..=last_id {
            let digits = id.ilog10() + 1;
            let div = 10_u64.pow(digits / 2);

            if digits % 2 != 0 {
                continue;
            }

            if id / div == id % div {
                sum += id;
            }
        }
    }

    sum
}

fn part2() -> u64 {
    let mut sum = 0;
    for id_ranges in INPUT.trim_end().split(',') {
        let (first_id, last_id): (u64, u64) = id_ranges
            .split_once('-')
            .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
            .unwrap();

        for id in first_id..=last_id {
            let digits = id.ilog10() + 1;

            'inner: for i in (1..=digits / 2).rev().filter(|i| digits % i == 0) {
                let mut sequences = (0..digits / i).map(|j| {
                    let div = 10_u64.pow(i * j);
                    let rem = 10_u64.pow(i);

                    (id / div) % rem
                });

                let num = sequences.next().unwrap();
                if sequences.all(|it| it == num) {
                    sum += id;
                    break 'inner;
                }
            }
        }
    }

    sum
}
