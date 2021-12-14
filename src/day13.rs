use aoc_runner_derive::aoc;

fn print_paper(paper: &Vec<Vec<bool>>) {
    for y in 0..paper[0].len() {
        for x in 0..paper.len() {
            if paper[x][y] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    let (dots, instructions) = input.split_once("\n\n").unwrap();
    // col major ordering
    let mut paper: Vec<Vec<bool>> = vec![];
    for line in dots.lines() {
        let dot = line.split_once(",").unwrap();
        let dot: (usize, usize) = (dot.0.parse().unwrap(), dot.1.parse().unwrap());
        if dot.0 >= paper.len() {
            let height = paper.get(0).map(|v| v.len()).unwrap_or(0);
            paper.resize_with(dot.0 + 1, || vec![false; height]);
        }
        if dot.1 >= paper[0].len() {
            for line in &mut paper {
                line.resize(dot.1 + 1, false);
            }
        }
        paper[dot.0][dot.1] = true;
    }
    // dbg!(&paper);
    for line in instructions.lines() {
        let fold = line.split_ascii_whitespace().skip(2).next().unwrap();
        let (dir, n) = fold.split_once("=").unwrap();
        let n = n.parse().unwrap();
        if dir == "y" {
            let len_folded = paper[0].len() - n - 1;
            let len_before = if n >= len_folded { n - len_folded } else { 0 };
            // dbg!(len_folded, len_before);
            for x in 0..paper.len() {
                for y in 0..paper[0].len() {
                    if y >= len_before && y < n {
                        // dbg!(n, (x, y), y + 2 * (n - y));
                        paper[x][y] = paper[x][y] || paper[x][y + 2 * (n - y)];
                    }
                }
            }
            for col in &mut paper {
                col.truncate(n);
            }
        }
        if dir == "x" {
            let len_folded = paper.len() - n - 1;
            let len_before = if n >= len_folded { n - len_folded } else { 0 };
            for x in 0..paper.len() {
                for y in 0..paper[0].len() {
                    if x >= len_before && x < n {
                        // dbg!(n, (x, y), x + 2 * (n - x));
                        paper[x][y] = paper[x][y] || paper[x + 2 * (n - x)][y];
                    }
                }
            }
            paper.truncate(n);
        }
        // dbg!(&paper);
        break; // remove for part 2
    }
    print_paper(&paper);
    paper.iter().fold(0, |total, col| {
        col.iter()
            .fold(total, |total, x| if *x { total + 1 } else { total })
    })
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample13.txt");
    assert_eq!(part1(&input), 17);
}