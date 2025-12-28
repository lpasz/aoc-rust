use std::collections::{HashSet, VecDeque};

// Simplificamos: O botão é apenas uma máscara de bits (um número)
#[derive(Debug)]
struct Puzzle {
    start_state: u32,
    goal_state: u32,
    buttons: Vec<u32>,
}

fn parse_optimized(input: &str) -> Vec<Puzzle> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line
                .split(['[', ']', '(', ')', '{', '}', ' '])
                .filter(|s| !s.is_empty())
                .collect();

            let switch_str = parts[0];
            let len = switch_str.len();

            let goal_state =
                u32::from_str_radix(&switch_str.replace('.', "0").replace('#', "1"), 2).unwrap();

            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|btn_str| {
                    btn_str.split(',').fold(0u32, |acc, idx_str| {
                        let idx: usize = idx_str.parse().unwrap();
                        acc | (1 << (len - 1 - idx))
                    })
                })
                .collect();

            Puzzle {
                start_state: 0,
                goal_state,
                buttons,
            }
        })
        .collect()
}

fn bfs_fast(puzzle: &Puzzle) -> Option<usize> {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    q.push_back((puzzle.start_state, 0));
    visited.insert(puzzle.start_state);

    while let Some((current_state, steps)) = q.pop_front() {
        if current_state == puzzle.goal_state {
            return Some(steps);
        }

        for &button_mask in &puzzle.buttons {
            let next_state = current_state ^ button_mask;

            if visited.insert(next_state) {
                q.push_back((next_state, steps + 1));
            }
        }
    }
    None
}

pub fn part1(input: &str) -> usize {
    let puzzles = parse_optimized(input);
    puzzles.iter().map(|p| bfs_fast(p).unwrap()).sum()
}

#[test]
fn part1_example() {
    let example = include_str!("../../assets/aoc25/day10/example.txt");
    assert_eq!(part1(example), 7);
}

#[test]
fn part1_input() {
    let input = include_str!("../../assets/aoc25/day10/input.txt");
    assert_eq!(part1(input), 517);
}
