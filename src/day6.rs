use aoc_runner_derive::aoc;

/// Initial naive solution.
///
/// Never think hard when the solution is simple
#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let mut fish: Vec<u32> = input.split(',').map(str::parse::<u32>).flatten().collect();
    for _ in 0..80 {
        let mut new_fish = 0;
        for f in &mut fish {
            match f {
                0 => {
                    new_fish += 1;
                    *f = 6;
                }
                _ => {
                    *f -= 1;
                }
            }
        }
        fish.resize(fish.len() + new_fish, 8);
    }
    fish.len()
}

/// But it's not that trivial today.
///
/// Turns out this was shorter, should have done it initially but I didn't think
/// of it.
#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let fish = input.split(',').map(str::parse::<usize>).flatten();
    let mut buckets = [0usize; 9];
    for f in fish {
        buckets[f] += 1;
    }
    for _ in 0..256 {
        let new_fish = buckets[0];
        for i in 0..8 {
            buckets[i] = buckets[i + 1];
        }
        buckets[8] = new_fish;
        buckets[6] += new_fish;
    }
    buckets.iter().sum()
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample6.txt");
    assert_eq!(part1(&input), 5934);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample6.txt");
    assert_eq!(part2(&input), 26984457539);
}
