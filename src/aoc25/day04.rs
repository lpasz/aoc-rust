use crate::core;

const INPUT: &str = include_str!("../../assets/aoc25/day04/input.txt");
const EXAMPLE: &str = include_str!("../../assets/aoc25/day04/example.txt");

fn part1(input: &str) -> i32 {
    let mtx: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut liftable_count = 0;

    for y in 0..mtx.len() {
        for x in 0..mtx[y].len() {
            if mtx[y][x] == '@' && rolls_around(&mtx, (x, y)) < 4 {
                liftable_count += 1;
            }
        }
    }

    liftable_count
}

fn rolls_around(mtx: &Vec<Vec<char>>, (x, y): (usize, usize)) -> usize {
    core::around((x, y))
        .into_iter()
        .filter_map(|(nx, ny)| mtx.get(ny)?.get(nx))
        .filter(|&&c| c == '@')
        .count()
}

fn part2(input: &str) -> i32 {
    let mut mtx: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut liftable_count = 0;
    let mut any_lifted = true;

    while any_lifted {
        any_lifted = false;
        for y in 0..mtx.len() {
            for x in 0..mtx[y].len() {
                if mtx[y][x] == '@' && rolls_around(&mtx, (x, y)) < 4 {
                    any_lifted = true;
                    mtx[y][x] = '.';
                    liftable_count += 1;
                }
            }
        }
    }

    liftable_count
}

#[test]
fn part1_example() {
    assert_eq!(part1(EXAMPLE), 13);
}

#[test]
fn part1_input() {
    assert_eq!(part1(INPUT), 1502);
}

#[test]
fn part2_example() {
    assert_eq!(part2(EXAMPLE), 43);
}

#[test]
fn part2_input() {
    assert_eq!(part2(INPUT), 9083);
}
