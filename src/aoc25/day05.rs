use std::ops::RangeInclusive;

fn parse(input: &str) -> (Vec<RangeInclusive<u128>>, impl Iterator<Item = u128> + '_) {
    let (fresh_ids, ids) = input.split_once("\n\n").unwrap();

    let mut fresh_ids: Vec<(u128, u128)> = fresh_ids
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(n1, n2)| {
            let n1: u128 = n1.parse().unwrap();
            let n2: u128 = n2.parse().unwrap();
            if n1 <= n2 {
                (n1, n2)
            } else {
                (n2, n1)
            }
        })
        .collect();

    fresh_ids.sort_unstable_by_key(|(n1, _)| *n1);

    let mut acc: Vec<RangeInclusive<u128>> = Vec::new();

    for (n1, n2) in fresh_ids {
        match acc.last_mut() {
            None => acc.push(n1..=n2),
            Some(prev) => {
                let prev_end = *prev.end();
                if n1 <= prev_end && n1 >= *prev.start() {
                    let start = *prev.start();
                    let end = prev_end.max(n2);
                    *prev = start..=end;
                } else {
                    acc.push(n1..=n2);
                }
            }
        }
    }

    let ids = ids
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap());

    (acc, ids)
}

pub fn part1(input: &str) -> usize {
    let (fresh_ids, ids) = parse(input);

    ids.filter(|id| fresh_ids.iter().any(|r| r.contains(id)))
        .count()
}

pub fn part2(input: &str) -> u128 {
    let (fresh_ids, _) = parse(input);

    fresh_ids.iter().map(|r| r.end() - r.start() + 1).sum()
}

#[test]
fn part1_example() {
    let example = include_str!("../../assets/aoc25/day05/example.txt");
    assert_eq!(part1(example), 3);
}

#[test]
fn part1_input() {
    let input = include_str!("../../assets/aoc25/day05/input.txt");
    assert_eq!(part1(input), 733);
}

#[test]
fn part2_example() {
    let example = include_str!("../../assets/aoc25/day05/example.txt");
    assert_eq!(part2(example), 14);
}

#[test]
fn part2_input() {
    let input = include_str!("../../assets/aoc25/day05/input.txt");
    assert_eq!(part2(input), 345821388687084);
}
