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
    Touch,
    Cross,
    Parallel,
    NoIntersection,
}

fn intersect_at_point(
    ((x1, y1), (x2, y2)): LineSegment,
    ((x3, y3), (x4, y4)): LineSegment,
) -> Intersection {
    let x1 = x1 as f32;
    let y1 = y1 as f32;
    let x2 = x2 as f32;
    let y2 = y2 as f32;
    let x3 = x3 as f32;
    let y3 = y3 as f32;
    let x4 = x4 as f32;
    let y4 = y4 as f32;
    let dx1 = x2 - x1;
    let dy1 = y2 - y1;
    let dx2 = x4 - x3;
    let dy2 = y4 - y3;

    let den = dx1 * dy2 - dy1 * dx2;

    if den == 0.0 {
        return Intersection::Parallel;
    }

    let t = ((x3 - x1) * dy2 - (y3 - y1) * dx2) / den;
    let u = ((x3 - x1) * dy1 - (y3 - y1) * dx1) / den;

    if t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0 {
        return Intersection::Cross;
    }

    if (t == 0.0 || t == 1.0 || u == 0.0 || u == 1.0)
        && (t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0)
    {
        return Intersection::Touch;
    } else {
        return Intersection::NoIntersection;
    }
}

fn lines_cross(linseg1: LineSegment, linseg2: LineSegment) -> bool {
    match intersect_at_point(linseg1, linseg2) {
        Intersection::Cross => true,
        // Intersection::Touch => true,
        _ => false,
    }
}

pub fn part2(input: &str) -> Option<usize> {
    let points = parse_points(input);

    let mut areas = vec![];
    for idx in 0..points.len() {
        for idx2 in idx + 1..points.len() {
            areas.push((
                points[idx],
                points[idx2],
                rectangle(points[idx], points[idx2]),
            ));
        }
    }

    areas.sort_unstable_by_key(|d| d.2);
    areas.reverse();

    let mut shifted: Vec<&Point> = points[1..].iter().collect();
    shifted.push(&points[0]);

    let edges: Vec<(&Point, &Point)> = points.iter().zip(shifted).collect();

    areas.iter().find_map(|((x1, y1), (x2, y2), area)| {
        let p1 = (*x1, *y1);
        let p2 = (*x1, *y2);
        let p3 = (*x2, *y2);
        let p4 = (*x2, *y1);

        let any_crossed = edges
            .iter()
            .flat_map(|(lsp1, lsp2)| {
                [
                    lines_cross((p1, p2), (**lsp1, **lsp2)),
                    lines_cross((p2, p3), (**lsp1, **lsp2)),
                    lines_cross((p3, p4), (**lsp1, **lsp2)),
                    lines_cross((p4, p1), (**lsp1, **lsp2)),
                    lines_cross((p1, p3), (**lsp1, **lsp2)),
                    lines_cross((p2, p4), (**lsp1, **lsp2)),
                ]
            })
            .any(|crossed| crossed);

        if any_crossed {
            None
        } else {
            Some(area.clone())
        }
    })
}

fn rectangle(p1: Point, p2: Point) -> usize {
    let dx = (p1.0 as i64 - p2.0 as i64).abs() + 1;
    let dy = (p1.1 as i64 - p2.1 as i64).abs() + 1;
    (dx * dy) as usize
}

#[test]
fn part1_example() {
    let example = include_str!("../../assets/aoc25/day09/example.txt");
    assert_eq!(part1(example), 50);
}

#[test]
fn part1_input() {
    let input = include_str!("../../assets/aoc25/day09/input.txt");
    assert_eq!(part1(input), 4746238001);
}

#[test]
fn part2_example() {
    let example = include_str!("../../assets/aoc25/day09/example.txt");
    assert_eq!(part2(example).unwrap(), 24);
}

#[test]
fn part2_input() {
    let input = include_str!("../../assets/aoc25/day09/input.txt");
    assert_eq!(part2(input).unwrap(), 1552139370);
}
