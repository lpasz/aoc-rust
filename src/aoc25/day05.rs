use std::ops::RangeInclusive;

fn parse(input: &str) -> (Vec<RangeInclusive<u128>>, Vec<u128>) {
    let (fresh_ids, ids) = input.split_once("\n\n").unwrap();

    let mut fresh_ids: Vec<(u128, u128)> = fresh_ids
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(n1, n2)| {
            let n1 = n1.parse().unwrap();
            let n2 = n2.parse().unwrap();

            if n1 < n2 {
                (n1, n2)
            } else {
                (n2, n1)
            }
        })
        .collect();

    fresh_ids.sort_unstable_by(|(n1, _), (n2, _)| n1.cmp(n2));

    let mut acc: Vec<RangeInclusive<u128>> = vec![];

    for (n1, n2) in fresh_ids {
        if acc.is_empty() {
            acc.push(n1..=n2);
        }

        if acc
            .last()
            .is_some_and(|r: &RangeInclusive<u128>| r.contains(&n1))
        {
            let prev = acc.pop().unwrap();
            let prev_start = prev.start();
            let prev_end = prev.end();
            acc.push(*prev_start.min(&n1)..=*prev_end.max(&n2));
        } else {
            acc.push(n1..=n2);
        }
    }

    let ids: Vec<u128> = ids.lines().map(|l| l.parse().unwrap()).collect();

    (acc, ids)
}

pub fn part1(input: &str) -> u128 {
    let (fresh_ids, ids) = parse(input);

    let mut sum = 0;
    for id in &ids {
        for fresh_id in &fresh_ids {
            if fresh_id.contains(&id) {
                sum += 1;
                break;
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> u128 {
    let (fresh_ids, _) = parse(input);

    let mut sum: u128 = 0;
    for fresh_id in fresh_ids {
        sum += fresh_id.count() as u128
    }
    sum
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
