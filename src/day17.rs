use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
fn parse(input: &str) -> ((isize, isize), (isize, isize)) {
    let mut input = input.split_ascii_whitespace().skip(2);
    let xrange: (isize, isize) = input
        .next()
        .unwrap()
        .split_once("=")
        .unwrap()
        .1
        .trim_end_matches(",")
        .split_once("..")
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .unwrap();
    let yrange: (isize, isize) = input
        .next()
        .unwrap()
        .split_once("=")
        .unwrap()
        .1
        .trim_end_matches(",")
        .split_once("..")
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .unwrap();
    (xrange, yrange)
}

#[aoc(day17, part1)]
fn part1(target: &((isize, isize), (isize, isize))) -> usize {
    let xvelocity = if target.0.0 < 0 {
        -1000..0
    } else {
        0..1000
    };
    let mut max = 0;
    for x in xvelocity {
        for y in 0..1000 {
            if let Some(height) = simulate((x, y), target) {
                max = usize::max(max, height as usize);
            }
        }
    }
    max
}

#[aoc(day17, part2)]
fn part2(target: &((isize, isize), (isize, isize))) -> usize {
    let xvelocity = if target.0.0 < 0 {
        -1000..0
    } else {
        0..1000
    };
    let mut count = 0;
    for x in xvelocity {
        for y in -1000..1000 {
            if simulate((x, y), target).is_some() {
                count += 1;
            }
        }
    }
    count
}

fn simulate(
    mut velocity: (isize, isize),
    target: &((isize, isize), (isize, isize)),
) -> Option<isize> {
    let mut pos = (0, 0);
    let mut max = 0;
    loop {
        max = isize::max(max, pos.1);
        if pos.0 >= target.0 .0
            && pos.0 <= target.0 .1
            && pos.1 >= target.1 .0
            && pos.1 <= target.1 .1
        {
            return Some(max);
        }
        if (pos.0 > 0 && pos.0 > target.0 .1) || (pos.0 < 0 && pos.0 < target.0 .0) {
            return None;
        }
        if pos.1 < target.1 .0 {
            return None;
        }
        pos.0 += velocity.0;
        pos.1 += velocity.1;
        if velocity.0 > 0 {
            velocity.0 -= 1;
        } else if velocity.0 < 0 {
            velocity.0 += 1;
        }
        velocity.1 -= 1;
    }
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample17.txt");
    assert_eq!(part1(&parse(&input)), 45);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample17.txt");
    assert_eq!(part2(&parse(&input)), 112);
}

