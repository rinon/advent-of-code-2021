use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
fn part1(input: &str) -> u32 {
    solve(input, false)
}

fn solve(input: &str, diagonals: bool) -> u32 {
    let mut size = (0, 0);
    let mut lines = vec![];
    for line in input.lines() {
        let mut words = line.split_ascii_whitespace();
        let mut pair = words.next().unwrap().split(',');
        let mut start = (
            pair.next().unwrap().parse().unwrap(),
            pair.next().unwrap().parse().unwrap(),
        );
        words.next();
        let mut pair = words.next().unwrap().split(',');
        let mut end = (
            pair.next().unwrap().parse().unwrap(),
            pair.next().unwrap().parse().unwrap(),
        );
        if (start.0 == end.0 && start.1 > end.1)
            || (start.1 == end.1 && start.0 > end.0)
            || (start.0 > end.0)
        {
            std::mem::swap(&mut start, &mut end);
        }
        if end.0 > size.0 {
            size = (end.0, size.1);
        }
        if end.1 > size.1 {
            size = (size.0, end.1);
        }
        lines.push((start, end));
    }

    let mut board = vec![vec![0u32; size.1 + 1]; size.0 + 1];
    for (start, end) in lines {
        // dbg!(start, end);
        if start.0 == end.0 {
            // vertical
            assert_ne!(start.1, end.1);
            for i in start.1..end.1 + 1 {
                board[start.0][i] += 1;
            }
        } else if start.1 == end.1 {
            // horizontal
            assert_ne!(start.0, end.0);
            for i in start.0..end.0 + 1 {
                board[i][start.1] += 1;
            }
        } else if diagonals && end.1 > start.1 {
            // diagonal down & right
            for i in 0..(end.0 - start.0 + 1) {
                board[start.0 + i][start.1 + i] += 1;
            }
        } else if diagonals {
            // diagonal up & right
            assert!(end.1 < start.1);
            for i in 0..(end.0 - start.0 + 1) {
                board[start.0 + i][start.1 - i] += 1;
            }
        }
    }
    // dbg!(&board);
    let mut count = 0;
    for col in board {
        for cell in col {
            if cell >= 2 {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u32 {
    solve(input, true)
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample5.txt");
    assert_eq!(part1(&input), 5);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample5.txt");
    assert_eq!(part2(&input), 12);
}
