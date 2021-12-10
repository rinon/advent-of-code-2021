use aoc_runner_derive::aoc;

fn parse(line: &str) -> Result<usize, Vec<char>> {
    let mut s = vec![];
    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => s.push(char),
            ')' if s.pop().unwrap() == '(' => {}
            ']' if s.pop().unwrap() == '[' => {}
            '}' if s.pop().unwrap() == '{' => {}
            '>' if s.pop().unwrap() == '<' => {}
            ')' => return Ok(3),
            ']' => return Ok(57),
            '}' => return Ok(1197),
            '>' => return Ok(25137),
            _ => panic!("Unexpected input character {}", char),
        }
    }
    Err(s)
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .fold(0, |total, line| total + parse(line).unwrap_or(0))
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let mut scores: Vec<_> = input.lines().filter_map(|line| {
        let s = parse(line);
        (match s {
            Ok(_) => None,
            Err(s) => {
                Some(s.iter().rev().fold(0, |score, c| {
                    (match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    }) + score * 5
                }))
            }
        })
    }).collect();
    scores.sort();
    scores[scores.len() / 2]
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample10.txt");
    assert_eq!(part1(&input), 26397);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample10.txt");
    assert_eq!(part2(&input), 288957);
}
