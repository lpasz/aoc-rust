use std::collections::HashSet;

type Point = (usize, usize, usize);

fn junction_boxes(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let n: Vec<usize> = l.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (n[0], n[1], n[2])
        })
        .collect()
}

fn euclidean_distance(p1: Point, p2: Point) -> usize {
    let dx = (p1.0 as i64 - p2.0 as i64).pow(2);
    let dy = (p1.1 as i64 - p2.1 as i64).pow(2);
    let dz = (p1.2 as i64 - p2.2 as i64).pow(2);
    ((dx + dy + dz) as f64).sqrt() as usize
}

fn get_sorted_distances(jbs: &[Point]) -> Vec<(usize, Point, Point)> {
    let mut distances = Vec::with_capacity((jbs.len() * (jbs.len() - 1)) / 2);
    for (idx, &p1) in jbs.iter().enumerate() {
        for &p2 in jbs.iter().skip(idx + 1) {
            distances.push((euclidean_distance(p1, p2), p1, p2));
        }
    }
    distances.sort_unstable_by_key(|d| d.0);
    distances
}

fn connect_points(sets: &mut Vec<HashSet<Point>>, p1: Point, p2: Point) {
    let mut first_match = None;
    let mut second_match = None;

    for (i, set) in sets.iter().enumerate() {
        if set.contains(&p1) || set.contains(&p2) {
            if first_match.is_none() {
                first_match = Some(i);
            } else {
                second_match = Some(i);
                break;
            }
        }
    }

    match (first_match, second_match) {
        (Some(i), Some(j)) => {
            let s2 = sets.remove(i.max(j));
            let mut s1 = sets.remove(i.min(j));
            s1.extend(s2);
            sets.push(s1);
        }
        (Some(i), None) => {
            sets[i].insert(p1);
            sets[i].insert(p2);
        }
        (None, None) => {
            let mut new_set = HashSet::new();
            new_set.insert(p1);
            new_set.insert(p2);
            sets.push(new_set);
        }
        _ => {}
    }
}

pub fn part1(input: &str, take_n: usize) -> usize {
    let jbs = junction_boxes(input);
    let distances = get_sorted_distances(&jbs);
    let mut sets: Vec<HashSet<Point>> = Vec::new();

    for (_, p1, p2) in distances.into_iter().take(take_n) {
        connect_points(&mut sets, p1, p2);
    }

    let mut sizes: Vec<usize> = sets.iter().map(|s| s.len()).collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}

pub fn part2(input: &str) -> usize {
    let jbs = junction_boxes(input);
    let mut distances = get_sorted_distances(&jbs);
    let mut sets: Vec<HashSet<Point>> = jbs.iter().map(|&p| [p].into_iter().collect()).collect();

    for (_, p1, p2) in distances.drain(..) {
        connect_points(&mut sets, p1, p2);

        if sets.len() == 1 {
            return p1.0 * p2.0;
        }
    }
    0
}

#[test]
fn part1_example() {
    let example = include_str!("../../assets/aoc25/day08/example.txt");
    assert_eq!(part1(example, 10), 40);
}

#[test]
fn part1_input() {
    let input = include_str!("../../assets/aoc25/day08/input.txt");
    assert_eq!(part1(input, 1000), 52668);
}

#[test]
fn part2_example() {
    let example = include_str!("../../assets/aoc25/day08/example.txt");
    assert_eq!(part2(example), 25272);
}

#[test]
fn part2_input() {
    let input = include_str!("../../assets/aoc25/day08/input.txt");
    assert_eq!(part2(input), 1474050600);
}
