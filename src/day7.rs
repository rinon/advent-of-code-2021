use aoc_runner_derive::aoc;

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let crabs: Vec<u32> = input.split(',').map(str::parse::<u32>).flatten().collect();
    let max = crabs.iter().max().unwrap();
    let mut min_fuel = usize::MAX;
    for i in 0..*max {
        let fuel = crabs
            .iter()
            .fold(0, |sum, crab| sum + (*crab as isize - i as isize).abs())
            as usize;
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let crabs: Vec<u32> = input.split(',').map(str::parse::<u32>).flatten().collect();
    let max = crabs.iter().max().unwrap();
    let mut min_fuel = usize::MAX;
    for i in 0..*max {
        let fuel = crabs.iter().fold(0, |sum, crab| {
            let n = (*crab as isize - i as isize).abs();
            sum + n * (n + 1) / 2
        }) as usize;
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample7.txt");
    assert_eq!(part1(&input), 37);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample7.txt");
    assert_eq!(part2(&input), 168);
}
