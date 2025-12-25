use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::core::{down, left, right, to_matrix};

pub fn part1(input: &str) -> usize {
    let mut mtx = to_matrix(input);

    let mut splitted: HashSet<(usize, usize)> = HashSet::new();
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();

    let (key, _value) = mtx.iter().find(|(_, &v)| v == 'S').unwrap();
    frontier.push_back(*key);

    while let Some(pos) = frontier.pop_front() {
        let Some(down_pos) = down(pos) else { continue };

        match mtx.get(&down_pos) {
            Some('.') => {
                mtx.insert(down_pos, '|');
                frontier.push_back(down_pos);
            }
            Some('^') => {
                splitted.insert(pos);
                for next in [left(down_pos), right(down_pos)].into_iter().flatten() {
                    match mtx.get(&next) {
                        Some('.') => {
                            mtx.insert(next, '|');
                            frontier.push_back(next);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    splitted.len()
}

pub fn part2(input: &str) -> usize {
    let mtx = to_matrix(input);
    let (key, _value) = mtx.iter().find(|(_, &v)| v == 'S').unwrap();
    bfs(&mtx, *key)
}

fn bfs(mtx: &HashMap<(usize, usize), char>, start_at: (usize, usize)) -> usize {
    let mut current_path_cnt: HashMap<(usize, usize), usize> = HashMap::new();
    current_path_cnt.insert(start_at, 1);

    let mut cnt = 0;

    loop {
        let mut next_path_cnt: HashMap<(usize, usize), usize> = HashMap::new();

        for (k, v) in current_path_cnt {
            if let Some(down) = down(k) {
                match mtx.get(&down) {
                    Some('.') => {
                        *next_path_cnt.entry(down).or_insert(0) += v;
                    }
                    Some('^') => {
                        *next_path_cnt.entry(left(down).unwrap()).or_insert(0) += v;
                        *next_path_cnt.entry(right(down).unwrap()).or_insert(0) += v;
                    }
                    _ => {
                        cnt += v;
                    }
                }
            }
        }

        if next_path_cnt.is_empty() {
            break;
        }
        current_path_cnt = next_path_cnt;
    }
    return cnt;
}

#[test]
fn part1_example() {
    let example = include_str!("../../assets/aoc25/day07/example.txt");
    assert_eq!(part1(example), 21);
}

#[test]
fn part1_input() {
    let input = include_str!("../../assets/aoc25/day07/input.txt");
    assert_eq!(part1(input), 1507);
}

#[test]
fn part2_example() {
    let example = include_str!("../../assets/aoc25/day07/example.txt");
    assert_eq!(part2(example), 40);
}

#[test]
fn part2_input() {
    let input = include_str!("../../assets/aoc25/day07/input.txt");
    assert_eq!(part2(input), 1537373473728);
}
