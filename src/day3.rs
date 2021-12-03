use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let bit_len = input.find('\n').unwrap();
    let input: Vec<u32> = input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();
    let input_len = input.len();
    let mut gamma = 0u32;
    for i in 0..32 {
        let mut one_count = 0u32;
        let mask = 1 << i;
        for x in &input {
            if x & mask != 0 {
                one_count += 1
            }
        }
        if one_count as usize > input_len / 2 {
            gamma |= mask;
        }
    }
    let epsilon = gamma ^ ((2 << (bit_len - 1)) - 1);
    epsilon * gamma
}

fn find_rating(mut input: Vec<u32>, bit_len: usize, criteria: bool) -> Option<u32> {
    for i in (0..bit_len).rev() {
        let mut one_count = 0u32;
        let mask = 1 << i;
        for x in &input {
            if x & mask != 0 {
                one_count += 1
            }
        }
        if (criteria && (one_count as usize >= (input.len() + 1) / 2))
            || (!criteria && (one_count as usize) < (input.len() + 1) / 2)
        {
            input.retain(|x| x & mask != 0);
        } else {
            input.retain(|x| x & mask == 0);
        }
        if input.len() == 1 {
            return Some(input[0]);
        }
    }
    None
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let bit_len = input.find('\n').unwrap();
    let input: Vec<u32> = input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();
    let oxy = find_rating(input.clone(), bit_len, true).unwrap();
    let co2 = find_rating(input, bit_len, false).unwrap();
    oxy * co2
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample3.txt");
    assert_eq!(part1(&input), 198);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample3.txt");
    assert_eq!(part2(&input), 230);
}
