const INPUT: &str = include_str!("input.txt");

fn main() {
    dbg!(part1());
    dbg!(part2());
}

fn part1() -> u32 {
    let mut sum = 0;

    for bank in INPUT.trim_end().lines() {
        let mut skip = 0;
        let mut a = '0';

        for (i, chr) in bank.char_indices().take(bank.len() - 1) {
            if chr > a {
                skip = i;
                a = chr;
            }
        }

        let mut b = '0';

        for chr in bank.chars().skip(skip + 1) {
            if chr > b {
                b = chr;
            }
        }

        sum += a
            .to_digit(10)
            .zip(b.to_digit(10))
            .map(|(a, b)| a * 10 + b)
            .unwrap();
    }

    sum
}

fn part2() -> u64 {
    let mut sum = 0;

    for bank in INPUT.trim_end().lines() {
        let mut joltage = 0;
        let mut skip = 0;

        for i in (0..12).rev() {
            let mut biggest = bank.chars().nth(skip).unwrap();

            let iter = bank.char_indices().take(bank.len() - i).skip(skip);
            for (i, chr) in iter {
                if chr > biggest {
                    skip = i;
                    biggest = chr;
                }
            }

            joltage += biggest.to_digit(10).unwrap() as u64 * 10_u64.pow(i as u32);
            skip += 1;
        }

        sum += joltage;
    }

    sum
}
