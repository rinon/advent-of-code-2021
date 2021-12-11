use aoc_runner_derive::aoc;
use std::collections::BTreeSet;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    input.lines().fold(0, |total, line| {
        line.split_ascii_whitespace()
            .skip_while(|x| *x != "|")
            .skip(1)
            .fold(0, |total, digit| match digit.len() {
                2 | 3 | 4 | 7 => total + 1,
                _ => total,
            })
            + total
    })
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    input.lines().fold(0, |total, line| {
        let mut input = line.split(" | ").map(str::split_ascii_whitespace);
        let patterns: Vec<BTreeSet<char>> =
            input.next().unwrap().map(|x| x.chars().collect()).collect();
        let output: Vec<BTreeSet<char>> = 
            input.next().unwrap().map(|x| x.chars().collect()).collect();

        let mut mappings: Vec<BTreeSet<char>> = vec![];
        for _ in 0..10 {
            mappings.push(BTreeSet::new());
        }

        for digit in &patterns {
            match digit.len() {
                2 => {
                    mappings[1].extend(digit.iter()); // 1
                }
                3 => {
                    mappings[7].extend(digit.iter()); // 7
                }
                4 => {
                    mappings[4].extend(digit.iter()); // 4
                }
                7 => {
                    mappings[8].extend(digit.iter());
                }
                _ => {}
            }
        }

        // We know 1, 4, 7, 8

        for digit in &patterns {
            match digit.len() {
                5 if mappings[1].is_subset(digit) => {
                    mappings[3].extend(digit.iter());
                }
                _ => {}
            }
        }

        // We know 1, 3, 4, 7, 8

        for digit in &patterns {
            match digit.len() {
                6 if !mappings[3]
                    .difference(&mappings[1])
                    .map(|x| *x)
                    .collect::<BTreeSet<char>>()
                    .is_subset(digit) =>
                {
                    mappings[0].extend(digit.iter());
                }
                6 if mappings[1].is_subset(digit) => {
                    mappings[9].extend(digit.iter());
                }
                _ => {}
            }
        }

        // We know 0, 1, 3, 4, 7, 8, 9

        for digit in &patterns {
            match digit.len() {
                5 if *digit == mappings[3] => {}
                5 if digit.is_subset(&mappings[9]) => {
                    mappings[5].extend(digit.iter());
                }
                5 => {
                    mappings[2].extend(digit.iter());
                }
                _ => {}
            }
        }

        // We know 0, 1, 2, 3, 4, 5, 7, 8, 9

        for digit in &patterns {
            match digit.len() {
                6 if *digit != mappings[0] && *digit != mappings[9] => {
                    mappings[6].extend(digit.iter());
                }
                _ => {}
            }
        }

        output.iter().fold(0, |total, digit| {
            total*10 + mappings.iter().position(|x| *x == *digit).unwrap()
        }) + total
    })
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample8.txt");
    assert_eq!(part1(&input), 26);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample8.txt");
    assert_eq!(part2(&input), 61229);
}
