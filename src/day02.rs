const INPUT: &str = include_str!("../assets/aoc25/day02/input.txt");
const EXAMPLE: &str = include_str!("../assets/aoc25/day02/example.txt");

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(&[',', '-'])
        .map(|i| i.trim())
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks_exact(2)
        .map(|c| (c[0], c[1]))
        .collect()
}

fn is_repeated_blocks(n: u64) -> bool {
    let digits = digits(n);

    for j in 2..=digits {
        if is_repeated_blocks_of_size(n, j) {
            return true;
        }
    }
    return false;
}

fn is_repeated_blocks_of_size(n: u64, size: u32) -> bool {
    if (0..10).contains(&n) {
        return false;
    }

    let digits = digits(n);

    if digits % size != 0 {
        return false;
    }

    let half: u32 = digits / size;

    let pow: u64 = u64::pow(10, half.try_into().unwrap());

    let right = n % pow;
    let mut left = n / pow;

    for _ in 2..size {
        let nn = left % pow;
        if nn != right {
            return false;
        }
        left = left / pow;
    }

    return left == right;
}

fn digits(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        (n.ilog10() + 1).into()
    }
}

fn part1(input: &str) -> u64 {
    let pairs = parse(input);
    let mut sum = 0;

    for (p1, p2) in pairs {
        for n in p1..=p2 {
            if is_repeated_blocks_of_size(n, 2) {
                sum += n
            }
        }
    }

    return sum;
}

fn part2(input: &str) -> u64 {
    let pairs = parse(input);
    let mut sum = 0;

    for (p1, p2) in pairs {
        for n in p1..=p2 {
            if is_repeated_blocks(n) {
                sum += n
            }
        }
    }

    return sum;
}

#[test]
fn part1_example() {
    assert_eq!(part1(EXAMPLE), 1227775554);
}

#[test]
fn part1_input() {
    assert_eq!(part1(INPUT), 20223751480);
}

#[test]
fn part2_example() {
    assert_eq!(part2(EXAMPLE), 4174379265);
}

#[test]
fn part2_input() {
    assert_eq!(part2(INPUT), 30260171216);
}
