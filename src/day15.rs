use aoc_runner_derive::aoc;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(unused)]
fn print_matrix(m: &[u32], width: usize) {
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
    if index >= width {
        n.push(index - width);
    }
    if index < width * (height - 1) {
        n.push(index + width);
    }
    if index % width != 0 {
        n.push(index - 1);
    }
    if (index + 1) % width != 0 {
        n.push(index + 1);
    }
    n
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    let width = input.split("\n").next().unwrap().len();
    let height = input.lines().count();

    let costs: Vec<_> = input.chars().filter_map(|x| x.to_digit(10)).collect();
    solve(&costs, width, height)
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
    let width = input.split("\n").next().unwrap().len();
    let height = input.lines().count();

    let costs: Vec<_> = input.chars().filter_map(|x| x.to_digit(10)).collect();
    let mut full_costs = vec![0; width * 5 * height * 5];

    for i in 0..5 {
        for j in 0..5 {
            for x in 0..width {
                for y in 0..height {
                    full_costs[i * width + j * width * 5 * height + x + y * width * 5] =
                        (costs[x + y * width] + i as u32 + j as u32 - 1) % 9 + 1;
                }
            }
        }
    }
    // print_matrix(&full_costs, width*5);
    solve(&full_costs, width * 5, height * 5)
}

fn solve(costs: &[u32], width: usize, height: usize) -> usize {
    let mut dist = vec![usize::MAX; width * height];
    let mut q = BinaryHeap::new();
    let start = 0;
    q.push(Reverse((0, start)));
    dist[start] = 0;
    let mut count = 0;
    while let Some(Reverse((cur_dist, i))) = q.pop() {
        for n in neighbors(i, width, height) {
            let next = dist[i] + costs[n] as usize;
            if next < dist[n] {
                dist[n] = next;
                q.push(Reverse((next, n)));
            }
        }
    }
    dist[width * height - 1]
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample15.txt");
    assert_eq!(part1(&input), 40);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample15.txt");
    assert_eq!(part2(&input), 2188189693529);
}
