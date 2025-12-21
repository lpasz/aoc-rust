const INPUT: &str = include_str!("../assets/aoc25/day01/input.txt");
const EXAMPLE: &str = include_str!("../assets/aoc25/day01/example.txt");

fn parse(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let symbol = chars.next().unwrap();
            let number = chars.as_str().parse::<i32>().unwrap();
            (symbol, number)
        })
        .collect()
}

fn part1(input: &str) -> i32 {
    let turns = parse(input);

    let mut dial = Dial::default();
    for turn in turns {
        dial.turn(turn);
    }

    return dial.finished_at_zero;
}

fn part2(input: &str) -> i32 {
    let turns = parse(input);

    let mut dial = Dial::default();
    for turn in turns {
        dial.turn(turn);
    }

    dial.visited_zero
}

#[derive(Debug)]
struct Dial {
    value: i32,
    finished_at_zero: i32,
    visited_zero: i32,
}

impl Dial {
    fn default() -> Self {
        Dial {
            finished_at_zero: 0,
            visited_zero: 0,
            value: 50,
        }
    }
    fn turn(&mut self, (dir, clicks): (char, i32)) -> &mut Self {
        println!("{:?}", self);
        if clicks == 0 {
            self.finished_at_zero += if self.value == 0 { 1 } else { 0 };
            return self;
        }

        match dir {
            'L' => self.value -= 1,
            'R' => self.value += 1,
            _ => panic!(),
        }

        match self.value {
            0 => {
                self.visited_zero += 1;
                self.turn((dir, clicks - 1))
            }
            100 => {
                self.value = 0;
                self.visited_zero += 1;
                self.turn((dir, clicks - 1))
            }
            -1 => {
                self.value = 99;
                self.turn((dir, clicks - 1))
            }
            _ => self.turn((dir, clicks - 1)),
        }
    }
}

#[test]
fn part1_example() {
    assert_eq!(part1(EXAMPLE), 3);
}

#[test]
fn part1_input() {
    assert_eq!(part1(INPUT), 1064);
}

#[test]
fn part2_example() {
    assert_eq!(part2(EXAMPLE), 6);
}

#[test]
fn part2_input() {
    assert_eq!(part2(INPUT), 6122);
}
