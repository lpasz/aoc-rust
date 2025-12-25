use std::collections::HashSet;

fn junction_boxes(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|l| (l[0], l[1], l[2]))
        .collect()
}

type Point = (usize, usize, usize);

fn euclidean_distance((x1, y1, z1): Point, (x2, y2, z2): Point) -> usize {
    let xmin = x1.min(x2);
    let ymin = y1.min(y2);
    let zmin = z1.min(z2);

    let xmax = x1.max(x2);
    let ymax = y1.max(y2);
    let zmax = z1.max(z2);

    let x = xmax - xmin;
    let y = ymax - ymin;
    let z = zmax - zmin;

    let sum = x.pow(2) + y.pow(2) + z.pow(2);
    sum.isqrt()
}

pub fn part1(input: &str, take_n: usize) -> usize {
    let jbs = junction_boxes(input);

    let mut distances = vec![];

    for (idx, jb1) in jbs.iter().enumerate() {
        for jb2 in jbs.iter().skip(idx + 1) {
            let distance = euclidean_distance(*jb1, *jb2);
            distances.push((distance, jb1, jb2));
        }
    }

    distances.sort_unstable_by_key(|d| d.0);

    let mut sets: Vec<HashSet<Point>> = vec![];
    for (_, p1, p2) in distances.into_iter().take(take_n) {
        // Encontramos quais sets atuais contêm p1 ou p2
        let mut first_match = None;
        let mut second_match = None;

        // Procuramos de trás para frente (removendo os sets que vamos fundir)
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
                // remove in order, to not cause shift on the other one
                let s2 = sets.remove(i.max(j));
                // remove the lowest after, this avoids shift
                let mut s1 = sets.remove(i.min(j));
                s1.extend(s2);
                sets.push(s1);
            }
            (Some(i), None) => {
                sets[i].insert(*p1);
                sets[i].insert(*p2);
            }
            (None, None) => {
                let mut new_set = HashSet::new();
                new_set.insert(*p1);
                new_set.insert(*p2);
                sets.push(new_set);
            }
            _ => {}
        }
    }

    // 3. Resultado final (Igual ao Elixir)
    let mut sizes: Vec<usize> = sets.iter().map(|s| s.len()).collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}

pub fn part2(input: &str) -> usize {
    let jbs = junction_boxes(input);

    let mut distances = vec![];

    for (idx, jb1) in jbs.iter().enumerate() {
        for jb2 in jbs.iter().skip(idx + 1) {
            let distance = euclidean_distance(*jb1, *jb2);
            distances.push((distance, jb1, jb2));
        }
    }

    distances.sort_unstable_by_key(|d| d.0);

    let mut sets: Vec<HashSet<Point>> = vec![];
    for jb in &jbs {
        let mut s = HashSet::new();
        s.insert(*jb);
        sets.push(s);
    }

    for (_d, p1, p2) in distances.drain(..) {
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
                // remove in order, to not cause shift on the other one
                let s2 = sets.remove(i.max(j));
                // remove the lowest after, this avoids shift
                let mut s1 = sets.remove(i.min(j));
                s1.extend(s2);
                sets.push(s1);
            }
            (Some(i), None) => {
                sets[i].insert(*p1);
                sets[i].insert(*p2);
            }
            (None, None) => {
                let mut new_set = HashSet::new();
                new_set.insert(*p1);
                new_set.insert(*p2);
                sets.push(new_set);
            }
            _ => {}
        }

        if sets.len() == 1 {
            return p1.0 * p2.0;
        }
    }

    return 0;
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
