const INPUT: &str = include_str!("../../assets/aoc25/day04/input.txt");
const EXAMPLE: &str = include_str!("../../assets/aoc25/day04/example.txt");

fn part1(input: &str) -> i32 {
    let mtx = input.lines().map(|l| l.chars());

    return 0;
}

fn more_4_around(mtx: Vec<Vec<char>>, i: usize, j: usize) -> bool {
    vec!([])
}

fn part2(input: &str) -> i32 {
    0
}

#[test]
fn part1_example() {
    assert_eq!(part1(EXAMPLE), 3);
}

#[test]
fn part1_input() {
    assert_eq!(part1(INPUT), 1064);
}

#[test]
fn part2_example() {
    assert_eq!(part2(EXAMPLE), 6);
}

#[test]
fn part2_input() {
    assert_eq!(part2(INPUT), 6122);
}
