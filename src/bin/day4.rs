#[derive(Debug, Clone)]
struct BingoBoard {
    rows: Vec<Vec<(u8, bool)>>,
    won: bool,
}

impl BingoBoard {
    fn new(rows: Vec<Vec<u8>>) -> Self {
        Self {
            rows: rows
                .iter()
                .map(|x| x.iter().map(|y| (*y, false)).collect())
                .collect(),
            won: false,
        }
    }
    fn check(&mut self, num: &u8) -> bool {
        for (value, stamp) in self.rows.iter_mut().flatten() {
            if *value == *num {
                *stamp = true;
                return true;
            }
        }
        false
    }
    fn is_winner(&mut self) -> bool {
        let mut column_check = vec![true; self.rows[0].len()];
        for row in self.rows.iter() {
            let mut row_check = true;
            for (i, (_, stamp)) in row.iter().enumerate() {
                if !stamp {
                    column_check[i] = false;
                    row_check = false;
                }
            }
            if row_check == true {
                self.won = true;
                return true;
            };
        }
        if column_check.contains(&true) {
            self.won = true;
            return true;
        };
        false
    }
    fn calc_unstamped(&self) -> i32 {
        let mut sum: i32 = 0;
        for (value, stamp) in self.rows.iter().flatten() {
            if !stamp {
                sum += i32::from(*value);
            }
        }
        sum
    }
}

fn input_parser(input: &str) -> (Vec<u8>, Vec<BingoBoard>) {
    let mut line_interator = input.lines();
    let mut bingo_board_list: Vec<BingoBoard> = Vec::new();
    let bingo_caller_list: Vec<u8> = line_interator
        .next()
        .unwrap()
        .split(",")
        .map(|val| val.parse().unwrap())
        .collect();

    line_interator.next();

    loop {
        let mut next_board_row_vect: Vec<Vec<u8>> = Vec::new();
        for _ in 0..=4 {
            let row: Vec<u8> = line_interator
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            next_board_row_vect.push(row);
        }
        bingo_board_list.push(BingoBoard::new(next_board_row_vect));

        if line_interator.next().is_none() {
            break;
        };
    }
    (bingo_caller_list, bingo_board_list)
}

fn solve_part_1(bingo_caller_list: &Vec<u8>, bingo_board_list: &Vec<BingoBoard>) -> i32 {
    let mut part_1_boards = bingo_board_list.clone();
    let mut winning_board: Option<BingoBoard> = None;
    let mut winning_number: Option<u8> = None;

    'main: for number in bingo_caller_list {
        for board in part_1_boards.iter_mut() {
            let was_stamped = board.check(number);
            if was_stamped {
                if board.is_winner() {
                    winning_board = Some(board.clone());
                    winning_number = Some(*number);
                    break 'main;
                }
            }
        }
    }
    winning_board.unwrap().calc_unstamped() * i32::from(winning_number.unwrap())
}
fn solve_part_2(bingo_caller_list: &Vec<u8>, bingo_board_list: &Vec<BingoBoard>) -> i32 {
    let mut part_2_boards = bingo_board_list.clone();
    let mut solutions: Vec<i32> = Vec::new();

    for number in bingo_caller_list {
        for board in part_2_boards.iter_mut() {
            if board.won {
                continue;
            }
            let was_stamped = board.check(number);
            if was_stamped {
                if board.is_winner() {
                    solutions.push(board.calc_unstamped() * i32::from(*number))
                }
            }
        }
    }
    *solutions.last().unwrap()
}

fn main() {
    let input = aoc2021::get_day_input(4);
    let (bingo_caller_list, bingo_board_list) = input_parser(&input);
    let part_1_result = solve_part_1(&bingo_caller_list, &bingo_board_list);
    println!("Solution to part one: {}", part_1_result);
    let part_2_result = solve_part_2(&bingo_caller_list, &bingo_board_list);
    println!("Solution to part two: {}", part_2_result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn col_winner_test() {
        let mut test = BingoBoard {
            rows: vec![
                vec![(0, false), (0, false), (0, false), (0, false), (0, true)],
                vec![(0, false), (0, false), (0, false), (0, false), (0, true)],
                vec![(0, false), (0, false), (0, false), (0, false), (0, true)],
                vec![(0, false), (0, false), (0, false), (0, false), (0, true)],
                vec![(0, false), (0, false), (0, false), (0, false), (0, true)],
            ],
            won: false,
        };
        assert_eq!(test.is_winner(), true);
        assert_eq!(test.won, true);
    }
    #[test]
    fn row_winner_test() {
        let mut test = BingoBoard {
            rows: vec![
                vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
                vec![(0, true), (0, true), (0, true), (0, true), (0, true)],
                vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
                vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
                vec![(0, false), (0, false), (0, false), (0, false), (0, false)],
            ],
            won: false,
        };
        assert_eq!(test.is_winner(), true);
        assert_eq!(test.won, true);
    }
    #[test]
    fn refactor_test() {
        let input = aoc2021::get_day_input(4);
        let (bingo_caller_list, bingo_board_list) = input_parser(&input);
        let part_1_result = solve_part_1(&bingo_caller_list, &bingo_board_list);
        assert_eq!(part_1_result, 51034);
        let part_2_result = solve_part_2(&bingo_caller_list, &bingo_board_list);
        assert_eq!(part_2_result, 5434);
    }
}
