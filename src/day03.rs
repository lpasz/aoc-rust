const INPUT: &str = include_str!("../assets/aoc25/day03/input.txt");
const EXAMPLE: &str = include_str!("../assets/aoc25/day03/example.txt");

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l: &str| l.chars().map(|c| c.to_digit(10).unwrap().into()).collect())
        .collect()
}

fn part1(input: &str) -> u128 {
    let nums = parse(input);

    let mut sum = 0;

    for line in nums {
        sum += max_joltage(line, 2);
    }

    return sum;
}

fn from_digits(digits: &Vec<u32>) -> u128 {
    let mut num: u128 = 0;

    for d in 0..digits.len() {
        num = num * 10 + digits[d] as u128
    }

    return num;
}

#[test]
fn test_from_digits() {
    assert_eq!(120, from_digits(&vec!(1, 2, 0)));
    assert_eq!(123456789, from_digits(&vec!(1, 2, 3, 4, 5, 6, 7, 8, 9)));
}

fn max_joltage(line: Vec<u32>, n: usize) -> u128 {
    let mut iter = line.into_iter();

    let mut start: Vec<u32> = iter.by_ref().take(n).collect();
    let rest: Vec<u32> = iter.collect();

    for n in rest {
        let mut idx: Option<_> = None;
        let mut max = from_digits(&start);

        for i in 0..start.len() {
            let mut imax: u128 = 0;
            let mut curr_idx = 0;
            for j in 0..start.len() {
                if i == j {
                    continue;
                }
                if i < j {
                    imax = imax * 10 + start[j] as u128;
                } else {
                    imax = imax * 10 + start[curr_idx] as u128;
                }
                curr_idx += 1;
            }
            imax = imax * 10 + n as u128;
            if imax > max {
                idx = Some(i);
                max = imax;
                break;
            }
        }

        if let Some(i) = idx {
            start.remove(i);
            start.push(n);
        }
    }

    from_digits(&start)
}

#[test]
fn test_max_joltage() {
    let v1 = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
    let v2 = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
    let v3 = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
    assert_eq!(987654321111, max_joltage(v1, 12));
    assert_eq!(811111111119, max_joltage(v2, 12));
    assert_eq!(434234234278, max_joltage(v3, 12));
}

fn part2(input: &str) -> u128 {
    let nums = parse(input);

    let mut sum = 0;

    for num in nums {
        sum += max_joltage(num, 12);
    }

    sum
}

#[test]
fn part1_example() {
    assert_eq!(part1(EXAMPLE), 357);
}

#[test]
fn part1_input() {
    assert_eq!(part1(INPUT), 17193);
}

#[test]
fn part2_example() {
    assert_eq!(part2(EXAMPLE), 3121910778619);
}

#[test]
fn part2_input() {
    assert_eq!(part2(INPUT), 171297349921310);
}
