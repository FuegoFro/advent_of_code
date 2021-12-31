use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter, Write};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum PodType {
    A,
    B,
    C,
    D,
}

impl PodType {
    fn target_room(&self) -> Room {
        match self {
            PodType::A => Room::A,
            PodType::B => Room::B,
            PodType::C => Room::C,
            PodType::D => Room::D,
        }
    }

    fn move_cost(&self) -> u32 {
        match self {
            PodType::A => 1,
            PodType::B => 10,
            PodType::C => 100,
            PodType::D => 1000,
        }
    }
}

impl PodType {
    fn from_char(c: char) -> Self {
        match c {
            'A' => PodType::A,
            'B' => PodType::B,
            'C' => PodType::C,
            'D' => PodType::D,
            _ => panic!("Unknown char {}", c),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Occupant {
    Empty,
    Pod(PodType),
}

impl Debug for Occupant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Occupant::Empty => '.',
            Occupant::Pod(pod_type) => match pod_type {
                PodType::A => 'A',
                PodType::B => 'B',
                PodType::C => 'C',
                PodType::D => 'D',
            },
        };
        f.write_char(char)
    }
}

#[derive(Copy, Clone)]
struct Room {
    front: usize,
    back: usize,
}

impl Room {
    const A: Room = Room::new(7, 8);
    const B: Room = Room::new(9, 10);
    const C: Room = Room::new(11, 12);
    const D: Room = Room::new(13, 14);

    const fn new(front: usize, back: usize) -> Self {
        Room { front, back }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Board {
    spots: Vec<Occupant>,
    cost_so_far: u32,
    estimated_remaining_cost: u32,
    parent: Option<Box<Board>>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Board {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .estimated_total_cost()
            .cmp(&self.estimated_total_cost())
            .then_with(|| self.spots.cmp(&other.spots))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/*
#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########

 01 2 3 4 56
   7 9 1 3
   8 0 2 4

*/
lazy_static! {
    static ref BOARD_NEIGHBORS: HashMap<usize, Vec<usize>> = [
        (0, vec![1]),
        (1, vec![0, 2, 7]),
        (2, vec![1, 3, 7, 9]),
        (3, vec![2, 4, 9, 11]),
        (4, vec![3, 5, 11, 13]),
        (5, vec![4, 6, 13]),
        (6, vec![5]),
        (7, vec![1, 2, 8]),
        (8, vec![7]),
        (9, vec![2, 3, 10]),
        (10, vec![9]),
        (11, vec![3, 4, 12]),
        (12, vec![11]),
        (13, vec![4, 5, 14]),
        (14, vec![13]),
    ]
    .into_iter()
    .collect();
    static ref BOARD_TARGET_STATE: Vec<Occupant> = vec![
        Occupant::Empty,
        Occupant::Empty,
        Occupant::Empty,
        Occupant::Empty,
        Occupant::Empty,
        Occupant::Empty,
        Occupant::Empty,
        Occupant::Pod(PodType::A),
        Occupant::Pod(PodType::A),
        Occupant::Pod(PodType::B),
        Occupant::Pod(PodType::B),
        Occupant::Pod(PodType::C),
        Occupant::Pod(PodType::C),
        Occupant::Pod(PodType::D),
        Occupant::Pod(PodType::D),
    ];
    static ref BOARD_DOUBLE_LEN_HOPS: HashSet<usize> =
        [1, 2, 3, 4, 5, 7, 9, 11, 13].into_iter().collect();
}

const BOARD_HALLWAYS_END: usize = 6;

impl Board {
    fn new(initial_room_assignments: &str) -> Self {
        let mut spots = vec![Occupant::Empty; BOARD_NEIGHBORS.len()];
        for (pos, c) in
            (BOARD_HALLWAYS_END + 1..BOARD_NEIGHBORS.len()).zip(initial_room_assignments.chars())
        {
            spots[pos] = Occupant::Pod(PodType::from_char(c));
        }
        let estimated_remaining_cost = Board::estimate_remaining_cost(&spots);
        Board {
            spots,
            cost_so_far: 0,
            estimated_remaining_cost,
            parent: None,
        }
    }

    fn with_move(&self, from: usize, to: usize, cost: u32) -> Self {
        let mut spots = self.spots.clone();
        spots.swap(from, to);
        let estimated_remaining_cost = Board::estimate_remaining_cost(&spots);
        Board {
            spots,
            cost_so_far: self.cost_so_far + cost,
            estimated_remaining_cost,
            parent: None,
            // parent: Some(Box::new(self.clone())),
        }
    }

    fn estimated_total_cost(&self) -> u32 {
        self.cost_so_far + self.estimated_remaining_cost
    }

    fn possible_moves(&self) -> Vec<Board> {
        // For each occupied spot
        // If at "back" of correct room, ignore
        // If correct room is available (reachable and empty or has sibling), move there.
        // If in room, move to each of the valid hallway spots
        let mut moves = Vec::new();
        for (pos, occupant) in self.spots.iter().enumerate() {
            match occupant {
                Occupant::Empty => continue,
                Occupant::Pod(pod_type) => {
                    let room = pod_type.target_room();
                    let reachable = Board::calculate_distances(Some(&self.spots), pos);
                    let back_has_correct_type = self.spots[room.back] == *occupant;
                    if pos == room.back || (pos == room.front && back_has_correct_type) {
                        // We're in the right place
                        continue;
                    }

                    if back_has_correct_type {
                        if let Some(path_len) = reachable.get(&room.front) {
                            // Move to the front spot
                            moves.push(self.with_move(
                                pos,
                                room.front,
                                path_len * pod_type.move_cost(),
                            ));
                        }
                    }
                    if self.spots[room.back] == Occupant::Empty {
                        if let Some(path_len) = reachable.get(&room.back) {
                            // Move to the back spot
                            moves.push(self.with_move(
                                pos,
                                room.back,
                                path_len * pod_type.move_cost(),
                            ));
                        }
                    }

                    if Board::is_in_room(pos) {
                        for hallway_pos in 0..=BOARD_HALLWAYS_END {
                            if let Some(path_len) = reachable.get(&hallway_pos) {
                                // Move to that position
                                moves.push(self.with_move(
                                    pos,
                                    hallway_pos,
                                    path_len * pod_type.move_cost(),
                                ));
                            }
                        }
                    }
                }
            }
        }
        moves
    }

    fn is_in_room(pos: usize) -> bool {
        pos > BOARD_HALLWAYS_END
    }

    /// Returns a map from reachable position to length of shortest path to get there.
    fn calculate_distances(spots: Option<&Vec<Occupant>>, pos: usize) -> HashMap<usize, u32> {
        let mut frontier = VecDeque::from([(pos, 0)]);
        let mut visited = HashSet::new();
        let mut reachable = HashMap::new();
        while let Some((next_pos, len_so_far)) = frontier.pop_front() {
            if visited.contains(&next_pos) {
                continue;
            }
            visited.insert(next_pos);
            if spots
                .map(|v| v[next_pos] == Occupant::Empty || next_pos == pos)
                .unwrap_or(true)
            {
                reachable.insert(next_pos, len_so_far);
                let was_on_double_len = BOARD_DOUBLE_LEN_HOPS.contains(&next_pos);
                for neighbor in BOARD_NEIGHBORS[&next_pos].iter() {
                    let move_len = if was_on_double_len && BOARD_DOUBLE_LEN_HOPS.contains(&neighbor)
                    {
                        2
                    } else {
                        1
                    };
                    frontier.push_back((*neighbor, len_so_far + move_len));
                }
            }
        }
        reachable
    }

    fn estimate_remaining_cost(spots: &Vec<Occupant>) -> u32 {
        let mut estimated_remaining_cost = 0;
        for (pos, occupant) in spots.iter().enumerate() {
            match occupant {
                Occupant::Empty => continue,
                Occupant::Pod(pod_type) => {
                    let room = pod_type.target_room();
                    if pos == room.back || (pos == room.front && spots[room.back] == *occupant) {
                        continue;
                    }
                    let estimated_distance = if pos == room.front {
                        // Minimal movement will be 4 spots to leave and come back in
                        4
                    } else {
                        // We know we're not in the right room, minimum movement will be going to
                        // the front of the correct room.
                        // TODO - we could memoize the non-spot-dependant path lengths
                        Board::calculate_distances(None, pos)[&room.front]
                    };
                    estimated_remaining_cost += estimated_distance * pod_type.move_cost();
                }
            }
        }
        estimated_remaining_cost
    }

    fn is_sorted(&self) -> bool {
        self.spots == *BOARD_TARGET_STATE
    }

    fn print(&self) {
        println!(
            "{:?}{:?}.{:?}.{:?}.{:?}.{:?}{:?}",
            self.spots[0],
            self.spots[1],
            self.spots[2],
            self.spots[3],
            self.spots[4],
            self.spots[5],
            self.spots[6]
        );
        println!(
            "  {:?} {:?} {:?} {:?}    cost: {}",
            self.spots[7], self.spots[9], self.spots[11], self.spots[13], self.cost_so_far
        );
        println!(
            "  {:?} {:?} {:?} {:?}    rem:  {}",
            self.spots[8],
            self.spots[10],
            self.spots[12],
            self.spots[14],
            self.estimated_remaining_cost
        );
    }

    fn print_ancestry(&self) {
        if let Some(parent) = &self.parent {
            parent.print_ancestry();
        }
        self.print();
        println!();
    }
}

fn search_cheapest_sort_path(initial_board: Board) -> u32 {
    let mut frontier = BinaryHeap::from([initial_board]);
    let mut seen_spots = HashSet::new();
    while let Some(board) = frontier.pop() {
        if seen_spots.contains(&board.spots) {
            // println!("SKIPPING DUPLICATE");
            // Board {
            //     spots: board.spots,
            //     cost_so_far: 0,
            //     estimated_remaining_cost: 0,
            // }
            // .print();
            continue;
        }
        // println!("---- PICKING BOARD ----");
        // board.print();
        if board.is_sorted() {
            board.print_ancestry();
            return board.cost_so_far;
        }

        frontier.extend(board.possible_moves());
        seen_spots.insert(board.spots);
    }
    panic!("Unable to find sort for board");
}

pub fn main() {
    // let input = "BACDBCDA";
    let input = "DCBAADCB";

    let initial_board = Board::new(input);

    println!("Part 1: {}", search_cheapest_sort_path(initial_board));
    // println!("Part 2: {}", "Answer here");
}
