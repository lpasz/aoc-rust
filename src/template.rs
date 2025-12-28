pub fn part1(input: &str) -> i32 {
    0
}

pub fn part2(input: &str) -> i32 {
    0
}

#[test]
fn part1_example() {
    let example = include_str!("../../assets/aoc25/day01/example.txt");
    assert_eq!(part1(example), 3);
}

#[test]
fn part1_input() {
    let input = include_str!("../../assets/aoc25/day01/input.txt");
    assert_eq!(part1(input), 1064);
}

#[test]
fn part2_example() {
    let example = include_str!("../../assets/aoc25/day01/example.txt");
    assert_eq!(part2(example), 6);
}

#[test]
fn part2_input() {
    let input = include_str!("../../assets/aoc25/day01/input.txt");
    assert_eq!(part2(input), 6122);
}
