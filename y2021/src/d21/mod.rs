use std::cmp::max;

#[derive(Debug)]
struct Player {
    position: u32,
    score: u32,
}

#[derive(Debug)]
struct Board {
    player1_turn: bool,
    next_num: u32,
    total_rolls: u32,
    player1: Player,
    player2: Player,
}

const DETERMINISTIC_WINNING_SCORE: u32 = 1000;
const DIRAC_WINNING_SCORE: u32 = 21;

const DICE_ROLLS: [(u32, u64); 7] = [
    // All possible outcomes of rolling a 3-sided die 3 times, with counts of each.
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1),
];

impl Board {
    fn new(player1_start_pos: u32, player2_start_pos: u32) -> Self {
        Board {
            player1_turn: true,
            next_num: 1,
            total_rolls: 0,
            player1: Player {
                position: player1_start_pos,
                score: 0,
            },
            player2: Player {
                position: player2_start_pos,
                score: 0,
            },
        }
    }

    fn current_player(&mut self) -> &mut Player {
        if self.player1_turn {
            &mut self.player1
        } else {
            &mut self.player2
        }
    }

    fn do_round_deterministic(&mut self) {
        for _ in 0..3 {
            self.current_player().position = (self.current_player().position + self.next_num) % 10;
            // Should range 1 to 100 inclusive
            self.next_num = (self.next_num % 100) + 1;
            self.total_rolls += 1;
        }
        self.current_player().score += self.current_player().position + 1;
        self.player1_turn = !self.player1_turn;
    }

    fn get_final_score(&self) -> Option<u32> {
        let lower_score = if self.player1.score >= DETERMINISTIC_WINNING_SCORE {
            Some(self.player2.score)
        } else if self.player2.score >= DETERMINISTIC_WINNING_SCORE {
            Some(self.player1.score)
        } else {
            None
        };
        lower_score.map(|s| s * self.total_rolls)
    }

    fn num_winning_possibilities(&mut self) -> (u64, u64) {
        if self.player1.score >= DIRAC_WINNING_SCORE {
            return (1, 0);
        } else if self.player2.score >= DIRAC_WINNING_SCORE {
            return (0, 1);
        }

        let mut p1_wins = 0;
        let mut p2_wins = 0;

        for (movement, count) in DICE_ROLLS.iter() {
            let old_position = self.current_player().position;
            self.current_player().position = (self.current_player().position + movement) % 10;
            self.current_player().score += self.current_player().position + 1;
            // This needs to be *just* before and after the recursive call since we use its value
            // in the `current_player()` function
            self.player1_turn = !self.player1_turn;

            let (p1_inner_wins, p2_inner_wins) = self.num_winning_possibilities();
            p1_wins += p1_inner_wins * count;
            p2_wins += p2_inner_wins * count;

            self.player1_turn = !self.player1_turn;
            self.current_player().score -= self.current_player().position + 1;
            self.current_player().position = old_position;
        }

        (p1_wins, p2_wins)
    }
}

pub fn main() {
    // let positions = (4, 8);
    let positions = (5, 9);

    let (p1_start, p2_start) = positions;
    let mut board = Board::new(p1_start - 1, p2_start - 1);

    while board.get_final_score().is_none() {
        board.do_round_deterministic();
    }

    println!("Part 1: {}", board.get_final_score().unwrap());

    let mut board = Board::new(p1_start - 1, p2_start - 1);
    let (p1_wins, p2_wins) = board.num_winning_possibilities();
    println!("Part 2: {}", max(p1_wins, p2_wins));
}
