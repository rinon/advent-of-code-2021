use aoc_runner_derive::aoc;

#[allow(unused)]
fn print_matrix(m: &[i32], width: usize) {
    for line in m.chunks(width) {
        println!(
            "{}",
            line.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("")
        );
    }
}

fn neighbors(index: usize, width: usize, height: usize) -> Vec<usize> {
    let mut n = vec![];
    let up = index >= width;
    let down = index < width * (height - 1);
    let left = index % width != 0;
    let right = (index + 1) % width != 0;
    if up {
        n.push(index - width);
    }
    if up && left {
        n.push(index - width - 1);
    }
    if up && right {
        n.push(index - width + 1);
    }
    if down {
        n.push(index + width);
    }
    if down && left {
        n.push(index + width - 1);
    }
    if down && right {
        n.push(index + width + 1);
    }
    if left {
        n.push(index - 1);
    }
    if right {
        n.push(index + 1);
    }
    n
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let width = 10;
    let height = 10;
    let mut octopuses: Vec<_> = input
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as i32)
        .collect();
    let mut flashes = 0;
    for _ in 0..100 {
        let mut flashing = vec![];
        for i in 0..octopuses.len() {
            octopuses[i] += 1;
            if octopuses[i] > 9 {
                octopuses[i] = -1;
                flashing.push(i);
            }
        }
        while let Some(i) = flashing.pop() {
            flashes += 1;
            for n in neighbors(i, width, height) {
                if octopuses[n] >= 0 {
                    octopuses[n] += 1;
                    if octopuses[n] > 9 {
                        octopuses[n] = -1;
                        flashing.push(n);
                    }
                }
            }
        }
        for i in 0..octopuses.len() {
            if octopuses[i] == -1 {
                octopuses[i] = 0;
            }
        }
        // print_matrix(&octopuses, 10);
        // println!();
    }
    flashes
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    let width = 10;
    let height = 10;
    let mut octopuses: Vec<_> = input
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as i32)
        .collect();
    let mut steps = 0;
    loop {
        let mut flashing = vec![];
        for i in 0..octopuses.len() {
            octopuses[i] += 1;
            if octopuses[i] > 9 {
                octopuses[i] = -1;
                flashing.push(i);
            }
        }
        while let Some(i) = flashing.pop() {
            for n in neighbors(i, width, height) {
                if octopuses[n] >= 0 {
                    octopuses[n] += 1;
                    if octopuses[n] > 9 {
                        octopuses[n] = -1;
                        flashing.push(n);
                    }
                }
            }
        }
        let mut all_flashing = true;
        for i in 0..octopuses.len() {
            if octopuses[i] == -1 {
                octopuses[i] = 0;
            } else {
                all_flashing = false;
            }
        }
        // print_matrix(&octopuses, 10);
        // println!();
        steps += 1;
        if all_flashing {
            break;
        }
    }
    steps
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample11.txt");
    assert_eq!(part1(&input), 1656);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample11.txt");
    assert_eq!(part2(&input), 195);
}
