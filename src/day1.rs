use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

fn count_increases(input: &[u32], window: usize) -> u32 {
    let mut count = 0;
    let first: u32 = input[0..window].iter().sum();
    input.windows(window).fold(first, |last, cur| {
        let cur = cur.iter().sum();
        if cur > last {
            count += 1;
        }
        cur
    });
    count
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    count_increases(input, 1)
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    count_increases(input, 3)
}

#[test]
fn test_part1() {
    assert_eq!(part1(&parse(include_str!("../input/2021/sample1.txt"))), 7);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&parse(include_str!("../input/2021/sample1.txt"))), 5);
}