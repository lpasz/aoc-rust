pub fn part1(input: &str) -> u128 {
    let lines = input.lines().map(|l| l.trim());

    let mut mtx: Vec<Vec<u128>> = vec![];
    let mut results: Vec<u128> = vec![];

    for line in lines {
        for (idx, n) in line.split_whitespace().map(|n| n.trim()).enumerate() {
            match n {
                "+" => results.push(mtx[idx].iter().sum()),
                "*" => results.push(mtx[idx].iter().product()),
                m => match mtx.get_mut(idx) {
                    Some(v) => v.push(n.parse().unwrap()),
                    None => mtx.push(vec![m.parse().unwrap()]),
                },
            }
        }
    }

    println!("mtx: {:?}", mtx);
    println!("results: {:?}", results);

    return results.iter().sum();
}

pub fn part2(input: &str) -> u128 {
    let lines = input.lines();

    let mut mtx: Vec<Vec<char>> = vec![];

    for (idx1, line) in lines.into_iter().enumerate() {
        for c in line.chars() {
            match mtx.get_mut(idx1) {
                Some(v) => v.push(c),
                None => mtx.push(vec![c]),
            }
        }
    }

    for line in mtx.iter_mut() {
        line.reverse();
    }

    let col_count = mtx[0].len();
    let mtx: Vec<Vec<char>> = (0..col_count)
        .map(|i| mtx.iter().map(|row| row[i].clone()).collect())
        .collect();

    println!("{:?}", mtx);
    let mut sum: u128 = 0;

    let mut nums: Vec<u128> = vec![];

    for line in mtx {
        let mut curr = vec![];
        for c in &line {
            match c.to_digit(10) {
                Some(n) => curr.push(n),
                None => {}
            }
        }
        if !curr.is_empty() {
            nums.push(crate::core::from_digits(curr));
        }

        println!("nums: {:?}, sum: {}", nums, sum);
        match line.last() {
            Some('*') => {
                sum += nums.iter().product::<u128>();
                nums = vec![];
            }
            Some('+') => {
                sum += nums.iter().sum::<u128>();
                nums = vec![];
            }
            _ => {}
        }

        println!("nums: {:?}, sum: {}", nums, sum);
    }
    sum
}

#[test]
fn part1_example() {
    let example = include_str!("../../assets/aoc25/day06/example.txt");
    assert_eq!(part1(example), 4277556);
}

#[test]
fn part1_input() {
    let input = include_str!("../../assets/aoc25/day06/input.txt");
    assert_eq!(part1(input), 6417439773370);
}

#[test]
fn part2_example() {
    let example = include_str!("../../assets/aoc25/day06/example.txt");
    assert_eq!(part2(example), 3263827);
}

#[test]
fn part2_input() {
    let input = include_str!("../../assets/aoc25/day06/input.txt");
    assert_eq!(part2(input), 11044319475191);
}
