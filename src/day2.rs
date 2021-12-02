use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Command, ()> {
        let mut words = s.split_whitespace();
        let command = words.next().ok_or(())?;
        let value = words.next().ok_or(())?.parse().or(Err(()))?;
        match command {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(str::parse::<Command>)
        .collect::<Result<Vec<Command>, ()>>()
        .unwrap()
}

#[aoc(day2, part1)]
fn part1(input: &[Command]) -> u32 {
    let mut pos = 0;
    let mut depth = 0;
    for command in input {
        match command {
            Command::Forward(x) => pos += x,
            Command::Down(x) => depth += x,
            Command::Up(x) => depth -= x,
        }
    }
    pos * depth
}

#[aoc(day2, part2)]
fn part2(input: &[Command]) -> u32 {
    let mut aim = 0;
    let mut pos = 0;
    let mut depth = 0;
    for command in input {
        match command {
            Command::Forward(x) => {
                pos += x;
                depth += aim * x;
            }
            Command::Down(x) => aim += x,
            Command::Up(x) => aim -= x,
        }
    }
    pos * depth
}

#[test]
fn test_part1() {
    let input = parse(include_str!("../input/2021/sample2.txt"));
    assert_eq!(part1(&input), 150);
}

#[test]
fn test_part2() {
    let input = parse(include_str!("../input/2021/sample2.txt"));
    assert_eq!(part2(&input), 900);
}