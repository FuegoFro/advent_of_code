use itertools::Itertools;
use std::collections::HashMap;
use util::p_u32;
use util::point::Point;

#[derive(Clone)]
struct Board {
    has_number: Vec<Vec<bool>>,
    number_positions: HashMap<u32, Point>,
}

impl Board {
    fn from_str(s: &str) -> Self {
        let mut has_number = vec![];
        let mut number_positions = HashMap::new();
        for (y, line) in s.split('\n').enumerate() {
            let mut has_number_row = vec![];
            for (x, num) in line.split_whitespace().map(p_u32).enumerate() {
                has_number_row.push(false);
                number_positions.insert(num, Point::new(x as i32, y as i32));
            }
            has_number.push(has_number_row)
        }
        Board {
            has_number,
            number_positions,
        }
    }

    fn add_number(&mut self, num: u32) {
        if let Some(pos) = self.number_positions.get(&num) {
            self.has_number[pos.y as usize][pos.x as usize] = true;
        }
    }

    fn has_completion(&self) -> bool {
        let has_complete_row = self.has_number.iter().any(|row| row.iter().all(|v| *v));
        let has_complete_col = (0..self.has_number[0].len()).any(|col_idx| {
            (0..self.has_number.len()).all(|row_idx| self.has_number[row_idx][col_idx])
        });
        has_complete_row || has_complete_col
    }

    fn compute_score(&self, final_num: u32) -> u32 {
        let sum_not_called: u32 = self
            .number_positions
            .iter()
            .filter_map(|(num, pos)| {
                if self.has_number[pos.y as usize][pos.x as usize] {
                    None
                } else {
                    Some(num)
                }
            })
            .sum();
        sum_not_called * final_num
    }
}

fn play_game(numbers: &[u32], mut boards: Vec<Board>, get_first_win: bool) -> (Board, u32) {
    for num in numbers.iter() {
        let mut winning_boards = vec![];
        let mut non_winning_boards = vec![];
        for mut board in boards.into_iter() {
            board.add_number(*num);
            let has_completion = board.has_completion();
            if has_completion {
                if get_first_win {
                    return (board, *num);
                } else {
                    winning_boards.push(board);
                }
            } else {
                non_winning_boards.push(board);
            }
        }
        if non_winning_boards.is_empty() && winning_boards.len() == 1 {
            return (winning_boards.into_iter().next().unwrap(), *num);
        }

        boards = non_winning_boards;
    }
    panic!("No board was completed!");
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let (raw_numbers, raw_boards) = input.split_once("\n\n").unwrap();
    let numbers = raw_numbers.split(',').map(p_u32).collect_vec();
    let boards = raw_boards.split("\n\n").map(Board::from_str).collect_vec();

    let (completed_board, final_num) = play_game(&numbers, boards.clone(), true);
    println!("Part 1: {}", completed_board.compute_score(final_num));

    let (completed_board, final_num) = play_game(&numbers, boards, false);
    println!("Part 2: {}", completed_board.compute_score(final_num));
}
