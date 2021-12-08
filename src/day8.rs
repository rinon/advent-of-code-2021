use aoc_runner_derive::aoc;
use std::collections::{BTreeSet, HashMap};

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

// fn assign(assignment: &mut HashMap<char, char>, digit: &str, values: &[char]) {
//     for (key, value) in digit.chars().zip(values.iter()) {
//         assert_eq!(assignment.insert(key, *value).unwrap_or(*value), *value);
//     }
// }

// fn intersect_possibilities(
//     possibilities: &mut HashMap<char, BTreeSet<char>>,
//     keys: &str,
//     digit: &str,
// ) {
//     for key in keys.chars() {
//         possibilities
//             .entry(key)
//             .and_modify(|p| p.retain(|x| digit.contains(*x)));
//     }
// }

// #[aoc(day8, part2)]
// fn part2(input: &str) -> usize {
//     input.lines().fold(0, |total, line| {
//         let mut input = line.split(" | ").map(str::split_ascii_whitespace);
//         let patterns: Vec<_> = input.next().unwrap().collect();
//         let output: Vec<_> = input.next().unwrap().collect();
//         let mut possibilities: HashMap<char, BTreeSet<char>> = HashMap::new();
//         for c in "abcdefg".chars() {
//             possibilities.insert(c, "abcdefg".chars().collect());
//         }
//         let mut assignments: HashMap<char, char> = HashMap::new();
//         let mut last_possibilities = possibilities.clone();
//         loop {
//             for digit in patterns.iter().chain(output.iter()) {
//                 match digit.len() {
//                     2 => {
//                         intersect_possibilities(&mut possibilities, digit, "cf");
//                         // 1
//                     }
//                     3 => {
//                         intersect_possibilities(&mut possibilities, digit, "acf");
//                         // 7
//                     }
//                     4 => {
//                         intersect_possibilities(&mut possibilities, digit, "bcdf");
//                         // 4
//                     }
//                     5 | 6 | 7 => {
//                         intersect_possibilities(&mut possibilities, digit, "abcdefg");
//                         // 0, 6, 9
//                     }
//                     n => panic!("Unexpected number of digits: {}", n),
//                 }
//             }

//             let mut connected: HashMap<Vec<char>, BTreeSet<char>> = HashMap::new();
//             for (key1, values1) in &possibilities {
//                 for (key2, values2) in &possibilities {
//                     if key1 != key2 && values1 == values2 {
//                         let values = values1.iter().map(|x| *x).collect();
//                         let connected_keys = connected.entry(values).or_default();
//                         connected_keys.insert(*key1);
//                         connected_keys.insert(*key2);
//                     }
//                 }
//             }
//             dbg!(&connected);
//             for (keys, values) in &connected {
//                 if keys.len() == values.len() {
//                     for (k, p) in &mut possibilities {
//                         if !values.contains(k) {
//                             p.retain(|x| !keys.contains(x));
//                         }
//                     }
//                 }
//             }
//             dbg!(&possibilities);

//             possibilities.retain(|k, p| {
//                 if p.len() == 1 {
//                     let value = *p.iter().next().unwrap();
//                     assert!(assignments.insert(*k, value).is_none());
//                 }
//                 p.len() > 1
//             });
//             for p in possibilities.values_mut() {
//                 p.retain(|x| assignments.values().find(|y| x == *y).is_none())
//             }
//             dbg!(&possibilities);

//             for digit in patterns.iter().chain(output.iter()) {
//                 match digit.len() {
//                     2 => {
//                         intersect_possibilities(&mut possibilities, digit, "cf");
//                         // 1
//                     }
//                     3 => {
//                         intersect_possibilities(&mut possibilities, digit, "acf");
//                         // 7
//                     }
//                     4 => {
//                         intersect_possibilities(&mut possibilities, digit, "bcdf");
//                         // 4
//                     }
//                     5 | 6 | 7 => {
//                         intersect_possibilities(&mut possibilities, digit, "abcdefg");
//                         // 0, 6, 9
//                     }
//                     n => panic!("Unexpected number of digits: {}", n),
//                 }
//             }
//             dbg!(&possibilities);

//             if possibilities == last_possibilities {
//                 break;
//             }
//             last_possibilities = possibilities.clone();
//         }
//         dbg!(&assignments);
//         // let mut assignments: HashMap<char, char> = HashMap::new();
//         // while !possibilities.is_empty() {
//         //     dbg!(&possibilities);
//         //     possibilities.retain(|key, possibilities| {
//         //         if possibilities.len() == 1 {
//         //             assignments.insert(*key, *possibilities.iter().next().unwrap());
//         //             false
//         //         } else {
//         //             true
//         //         }
//         //     });
//         //     for (key, assignment) in &assignments {
//         //         for (_, possibilities) in &mut possibilities {
//         //             possibilities.retain(|x| x != assignment);
//         //         }
//         //     }
//         // }
//         total
//     })
// }

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
        let mut assignments: HashMap<char, char> = HashMap::new();

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

        let a = *mappings[7]
            .difference(&mappings[1])
            .next()
            .unwrap();

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
