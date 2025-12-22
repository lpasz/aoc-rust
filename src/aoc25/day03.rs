pub fn part1(input: &str) -> u128 {
    max_joltage_with_n_batteries(input, 2)
}

pub fn part2(input: &str) -> u128 {
    max_joltage_with_n_batteries(input, 12)
}

fn from_digits(digits: Vec<u32>) -> u128 {
    digits.into_iter().fold(0, |num, d| num * 10 + d as u128)
}

fn max_joltage_with_n_batteries(input: &str, n: usize) -> u128 {
    input.lines().map(|l| max_joltage(l, n)).sum()
}

fn max_joltage(line: &str, number_of_batteries_to_pick: usize) -> u128 {
    let total_num_of_batteries = line.len();
    let batteries = line.chars().filter_map(|c| c.to_digit(10));
    let mut selected_batteries: Vec<u32> = Vec::with_capacity(number_of_batteries_to_pick);
    let number_of_batteries_to_reject = total_num_of_batteries - number_of_batteries_to_pick;
    let mut number_of_batteries_rejected = 0;

    for digit in batteries {
        while number_of_batteries_rejected < number_of_batteries_to_reject
            && selected_batteries.last().is_some_and(|last| last < &digit)
        {
            selected_batteries.pop();
            number_of_batteries_rejected += 1;
        }

        if selected_batteries.len() < number_of_batteries_to_pick {
            selected_batteries.push(digit);
        } else {
            number_of_batteries_rejected += 1;
        }
    }

    from_digits(selected_batteries)
}

#[test]
fn part1_example() {
    let example = include_str!("../../assets/aoc25/day03/example.txt");

    assert_eq!(part1(example), 357);
}

#[test]
fn part1_input() {
    let input = include_str!("../../assets/aoc25/day03/input.txt");

    assert_eq!(part1(input), 17193);
}

#[test]
fn part2_example() {
    let example = include_str!("../../assets/aoc25/day03/example.txt");

    assert_eq!(part2(example), 3121910778619);
}

#[test]
fn part2_input() {
    let input = include_str!("../../assets/aoc25/day03/input.txt");

    assert_eq!(part2(input), 171297349921310);
}
