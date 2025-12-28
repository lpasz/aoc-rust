use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct IndicatorLights {
    final_result: usize,
    lights_on: usize,
    len: usize,
    presses: usize,
}

#[derive(Debug)]
struct Button {
    indicators: Vec<usize>,
}

impl IndicatorLights {
    fn after_button_press(self: &Self, button: &Button) -> IndicatorLights {
        // println!("{:?} {:?}", self, button);
        let mut cnt = self.lights_on;
        for i in &button.indicators {
            let shift = 1 << (self.len - i);
            cnt = cnt ^ shift;
            // println!("{:b} <-> {:b} <-> {:b}", self.lights_on, cnt, shift);
        }

        let updated = Self {
            final_result: self.final_result,
            lights_on: cnt,
            len: self.len,
            presses: self.presses + 1,
        };

        // println!("{:?}", updated);
        updated
    }
}

fn parse(input: &str) -> Vec<(IndicatorLights, Vec<Button>)> {
    let mut result = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line
            .split(['[', ']', '(', ')', '{', '}', ' '])
            .filter(|s| *s != "")
            .collect();
        let s = parts[0];
        let mid = &parts[1..(parts.len() - 1)];

        let s = s.replace('.', "0").replace('#', "1");

        let button_presses: Vec<Button> = mid
            .iter()
            .map(|n| Button {
                indicators: n.split(',').map(|n| n.parse().unwrap()).collect(),
            })
            .collect();

        result.push((
            IndicatorLights {
                final_result: usize::from_str_radix(s.as_str(), 2).unwrap(),
                lights_on: 0,
                len: s.len() - 1,
                presses: 0,
            },
            button_presses,
        ));
    }

    result
}

fn bfs(b: IndicatorLights, bps: Vec<Button>) -> Option<usize> {
    let mut q = VecDeque::new();
    let mut s = HashSet::new();
    s.insert(b.lights_on);
    q.push_front(b);

    while let Some(b) = q.pop_back() {
        for bp in &bps {
            let after_press = b.after_button_press(&bp);

            if after_press.final_result.eq(&after_press.lights_on) {
                return Some(after_press.presses);
            }

            if s.insert(after_press.lights_on) {
                q.push_front(after_press);
            }
        }
    }

    None
}

pub fn part1(input: &str) -> usize {
    let p = parse(input);
    let mut s = 0;
    for (b, bps) in p {
        s += bfs(b, bps).unwrap();
    }
    s
}

pub fn part2(input: &str) -> i32 {
    0
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

// #[test]
// fn part2_example() {
//     let example = include_str!("../../assets/aoc25/day10/example.txt");
//     assert_eq!(part2(example), 6);
// }

// #[test]
// fn part2_input() {
//     let input = include_str!("../../assets/aoc25/day10/input.txt");
//     assert_eq!(part2(input), 6122);
// }
//
#[test]
fn indicator_lights() {
    let il = IndicatorLights {
        final_result: 0b0110,
        lights_on: 0,
        len: 3,
        presses: 0,
    }
    .after_button_press(&Button {
        indicators: vec![0, 2],
    })
    .after_button_press(&Button {
        indicators: vec![0, 1],
    });

    assert_eq!(il.presses, 2);
    assert_eq!(il.lights_on, il.final_result);

    let il = IndicatorLights {
        final_result: 0b00010,
        lights_on: 0,
        len: 4,
        presses: 0,
    }
    .after_button_press(&Button {
        indicators: vec![0, 4],
    })
    .after_button_press(&Button {
        indicators: vec![0, 1, 2],
    })
    .after_button_press(&Button {
        indicators: vec![1, 2, 3, 4],
    });

    assert_eq!(il.presses, 3);
    assert_eq!(il.lights_on, il.final_result);

    let il = IndicatorLights {
        final_result: 0b011101,
        lights_on: 0,
        len: 5,
        presses: 0,
    }
    .after_button_press(&Button {
        indicators: vec![0, 3, 4],
    })
    .after_button_press(&Button {
        indicators: vec![0, 1, 2, 4, 5],
    });

    assert_eq!(il.presses, 2);
    assert_eq!(il.lights_on, il.final_result);
}
