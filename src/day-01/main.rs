const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("part1 = {}", part1());
    println!("part2 = {}", part2());
}

fn part1() -> u16 {
    let mut pass: u16 = 0;
    let mut dial: u8 = 50;

    for line in INPUT.trim_end().lines() {
        let (direction, num) = line.split_at(1);
        let distance = (num.parse::<u16>().unwrap() % 100) as u8;

        match direction {
            "L" => dial += 100 - distance,
            "R" => dial += distance,
            _ => unreachable!(),
        };

        dial %= 100;

        if dial == 0 {
            pass += 1;
        }
    }

    pass
}

fn part2() -> u16 {
    let mut pass: u16 = 0;
    let mut dial: u8 = 50;
    let mut zero = dial == 0;

    for line in INPUT.trim_end().lines() {
        let (direction, num) = line.split_at(1);
        let num = num.parse::<u16>().unwrap();
        pass += num / 100;
        let distance = (num % 100) as u8;

        match direction {
            "L" => {
                dial += 100 - distance;
                if dial <= 100 && !zero {
                    pass += 1;
                }
            }
            "R" => {
                dial += distance;
                if dial >= 100 && !zero {
                    pass += 1;
                }
            }
            _ => unreachable!(),
        };

        dial %= 100;
        zero = dial == 0;
    }

    pass
}
