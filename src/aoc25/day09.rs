fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let n: Vec<usize> = l.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (n[0], n[1])
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let points = parse_points(input);

    let mut max = 0;
    for (idx, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(idx + 1) {
            max = rectangle(*p1, *p2).max(max)
        }
    }

    max
}

type Point = (usize, usize);
type LineSegment = (Point, Point);
enum Intersection {
    Touch(Point),
    Cross(Point),
    Parallel,
    NoIntersection,
}

fn intersect_at_point(
    ((x1, y1), (x2, y2)): LineSegment,
    ((x3, y3), (x4, y4)): LineSegment,
) -> Intersection {
    let dx1 = x2 - x1;
    let dy1 = y2 - y1;
    let dx2 = x4 - x3;
    let dy2 = y4 - y3;

    let den = dx1 * dy2 - dy1 * dx2;

    if den == 0 {
        return Intersection::Parallel;
    }

    let t = ((x3 - x1) * dy2 - (y3 - y1) * dx2) / den;
    let u = ((x3 - x1) * dy1 - (y3 - y1) * dx1) / den;

    if t > 0 && t < 1 && u > 0 && u < 1 {
        return Intersection::Cross((x1 + t * dx1, y1 + t * dy1));
    }

    if (t == 0 || t == 1 || u == 0 || u == 1) && (t >= 0 && t <= 1 && u >= 0 && u <= 1) {
        return Intersection::Touch((x1 + t * dx1, y1 + t * dy1));
    } else {
        return Intersection::NoIntersection;
    }
}

fn lines_cross(linseg1: LineSegment, linseg2: LineSegment) -> bool {
    match intersect_at_point(linseg1, linseg2) {
        Intersection::Cross(_) => true,
        _ => false,
    }
}

pub fn part2(input: &str) -> i32 {
    let points = parse_points(input);

    let mut areas = vec![];
    for (idx, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(idx + 1) {
            areas.push((p1, p2, rectangle(*p1, *p2)));
        }
    }

    areas.sort_unstable_by_key(|d| d.2).rev();

    let shifted: Vec<_> = points[1..].iter().collect();
    shifted.push(&points[0]);

    let edges = points.iter().zip(shifted).collect();

    areas.iter().find_map(|((x1, y1), (x2, y2), area)| {
        let p1 = (*x1, *y1);
        let p2 = (*x1, *y2);
        let p3 = (*x2, *y2);
        let p4 = (*x2, *y1);

        for linseg2 in edges {
            if !lines_cross((p1, p2), linseg2)
                && !lines_cross((p2, p3), linseg2)
                && !lines_cross((p3, p4), linseg2)
                && !lines_cross((p4, p1), linseg2)
                && !lines_cross((p1, p3), linseg2)
                && !lines_cross((p2, p4), linseg2)
            {
                return Some(area);
            }
        }
        None
    });
}

fn rectangle(p1: Point, p2: Point) -> usize {
    let dx = (p1.0 as i64 - p2.0 as i64).abs() + 1;
    let dy = (p1.1 as i64 - p2.1 as i64).abs() + 1;
    (dx * dy) as usize
}

#[test]
fn part1_example() {
    let example = include_str!("../../assets/aoc25/day09/example.txt");
    assert_eq!(part1(example), 3);
}

#[test]
fn part1_input() {
    let input = include_str!("../../assets/aoc25/day09/input.txt");
    assert_eq!(part1(input), 1064);
}

#[test]
fn part2_example() {
    let example = include_str!("../../assets/aoc25/day09/example.txt");
    assert_eq!(part2(example), 6);
}

#[test]
fn part2_input() {
    let input = include_str!("../../assets/aoc25/day09/input.txt");
    assert_eq!(part2(input), 6122);
}
