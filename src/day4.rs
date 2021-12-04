use aoc_runner_derive::aoc;

#[derive(Copy, Clone, Debug, Default)]
struct Cell {
    num: u32,
    marked: bool,
}

#[derive(Clone, Debug)]
struct Board {
    cells: [Cell; 25],
    won: bool,
}

#[derive(Clone, Debug)]
struct Game {
    draws: Vec<u32>,
    boards: Vec<Board>,
}

impl Game {
    fn new(input: &str) -> Game {
        let mut lines = input.lines();
        let mut draws: Vec<u32> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        draws.reverse();
        lines.next();
        let lines: Vec<&str> = lines.collect();
        let mut boards = Vec::new();
        for board_input in lines.split(|x| x.is_empty()) {
            let mut cells = [Cell::default(); 25];
            let mut nums = board_input
                .into_iter()
                .map(|line| line.split_whitespace())
                .flatten()
                .map(|x| Cell {
                    num: x.parse().unwrap(),
                    marked: false,
                });
            cells.fill_with(|| nums.next().unwrap());
            boards.push(Board { cells, won: false });
        }
        Game { draws, boards }
    }

    fn play_bingo(&mut self) -> Option<u32> {
        while let Some(num) = self.draws.pop() {
            // dbg!(num);
            for board in &mut self.boards {
                if let Some(cell) = board.cells.iter_mut().find(|x| x.num == num) {
                    cell.marked = true;
                }
            }
            if self.check_new_bingo() {
                return Some(num);
            }
        }
        None
    }

    fn check_new_bingo(&mut self) -> bool {
        let mut found_new_bingo = false;
        for board in &mut self.boards {
            board
                .cells
                .chunks(5)
                .any(|row| row.iter().all(|x| x.marked));
            for i in 0..5 {
                if board.cells[i * 5..i * 5 + 5].iter().all(|x| x.marked) {
                    println!("row bingo");
                    if !board.won {
                        found_new_bingo = true;
                    }
                    board.won = true;
                }
                let mut count = 0;
                for j in 0..5 {
                    if board.cells[i + 5 * j].marked {
                        count += 1;
                    }
                }
                if count == 5 {
                    println!("col bingo");
                    if !board.won {
                        found_new_bingo = true;
                    }
                    board.won = true;
                }
            }
            // Apparently diagonals don't count...
            //
            // if board[0] && board[6] && board[12] && board[18] && board[24] {
            //     println!("diag bingo in {:?}", board);
            //     return true;
            // }
            // if board[4] && board[8] && board[12] && board[16] && board[20] {
            //     println!("diag bingo in {:?}", board);
            //     return true;
            // }
        }
        found_new_bingo
    }
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let mut game = Game::new(input);
    let last = game.play_bingo().unwrap();
    // dbg!(last);
    game.boards
        .iter()
        .find(|board| board.won)
        .unwrap()
        .cells
        .iter()
        .fold(
            0,
            |total, cell| if !cell.marked { total + cell.num } else { total },
        )
        * last
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    let mut game = Game::new(input);
    let mut board_won = vec![false; game.boards.len()];
    let mut last_num = 0;
    let mut last_index = 0;
    while let Some(num) = game.play_bingo() {
        for (index, board) in game.boards.iter().enumerate() {
            if board.won && !board_won[index] {
                last_num = num;
                last_index = index;
                board_won[index] = true;
            }
        }
        // dbg!(last_num, last_index);
        // dbg!(&game.marks);
        // dbg!(&board_won);
        if game.boards.iter().all(|board| board.won) {
            break;
        }
    }
    // dbg!(&game.boards[last_index]);
    // dbg!(last_num, last_index);
    game.boards[last_index].cells.iter().fold(
        0,
        |total, cell| if !cell.marked { total + cell.num } else { total },
    ) * last_num
}

#[test]
fn test_part1() {
    let input = include_str!("../input/2021/sample4.txt");
    assert_eq!(part1(&input), 4512);
}

#[test]
fn test_part2() {
    let input = include_str!("../input/2021/sample4.txt");
    assert_eq!(part2(&input), 1924);
}
