use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter, Write};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
    positions: [usize; 4],
}

impl Room {
    const A: Room = Room::new([7, 11, 15, 19]);
    const B: Room = Room::new([8, 12, 16, 20]);
    const C: Room = Room::new([9, 13, 17, 21]);
    const D: Room = Room::new([10, 14, 18, 22]);

    const fn new(positions: [usize; 4]) -> Self {
        Room { positions }
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
   7 8 9 0
   1 2 3 4
   5 6 7 8
   9 0 1 2


*/
lazy_static! {
    static ref BOARD_NEIGHBORS: HashMap<usize, Vec<usize>> = [
        (0, vec![1]),
        (1, vec![0, 2, 7]),
        (2, vec![1, 3, 7, 8]),
        (3, vec![2, 4, 8, 9]),
        (4, vec![3, 5, 9, 10]),
        (5, vec![4, 6, 10]),
        (6, vec![5]),
        (7, vec![1, 2, 11]),
        (8, vec![2, 3, 12]),
        (9, vec![3, 4, 13]),
        (10, vec![4, 5, 14]),
        (11, vec![7, 15]),
        (12, vec![8, 16]),
        (13, vec![9, 17]),
        (14, vec![10, 18]),
        (15, vec![11, 19]),
        (16, vec![12, 20]),
        (17, vec![13, 21]),
        (18, vec![14, 22]),
        (19, vec![15]),
        (20, vec![16]),
        (21, vec![17]),
        (22, vec![18]),
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
        Occupant::Pod(PodType::B),
        Occupant::Pod(PodType::C),
        Occupant::Pod(PodType::D),
        Occupant::Pod(PodType::A),
        Occupant::Pod(PodType::B),
        Occupant::Pod(PodType::C),
        Occupant::Pod(PodType::D),
        Occupant::Pod(PodType::A),
        Occupant::Pod(PodType::B),
        Occupant::Pod(PodType::C),
        Occupant::Pod(PodType::D),
        Occupant::Pod(PodType::A),
        Occupant::Pod(PodType::B),
        Occupant::Pod(PodType::C),
        Occupant::Pod(PodType::D),
    ];
    static ref BOARD_DOUBLE_LEN_HOPS: HashSet<usize> =
        [1, 2, 3, 4, 5, 7, 8, 9, 10].into_iter().collect();
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
                    // room.contains(pos)
                    // self.room_matches_type(pod_type)
                    // self.farthest_back_open_spot(room)
                    if Board::room_matches_type_behind(&self.spots, &room, *pod_type, pos) {
                        // We're in the right place
                        continue;
                    }

                    if self.room_matches_type(&room, *pod_type) {
                        let target_spot = self.farthest_back_open_spot(&room).unwrap();
                        if let Some(path_len) = reachable.get(&target_spot) {
                            // Move to the spot
                            moves.push(self.with_move(
                                pos,
                                target_spot,
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
                    if Board::room_matches_type_behind(&spots, &room, *pod_type, pos) {
                        continue;
                    }
                    let estimated_distance =
                        if let Some(room_idx) = room.positions.iter().position(|v| *v == pos) {
                            // Minimal movement will be 4 spots to leave and come back in, plus
                            // however many to get to the front of the room.
                            4 + room_idx as u32
                        } else {
                            // We know we're not in the right room, minimum movement will be going to
                            // the front of the correct room.
                            // TODO - we could memoize the non-spot-dependant path lengths
                            Board::calculate_distances(None, pos)[&room.positions[0]]
                        };
                    estimated_remaining_cost += estimated_distance * pod_type.move_cost();
                }
            }
        }
        estimated_remaining_cost
    }

    fn room_matches_type(&self, room: &Room, pod_type: PodType) -> bool {
        room.positions.iter().all(|idx| match self.spots[*idx] {
            Occupant::Empty => true,
            Occupant::Pod(pt) => pt == pod_type,
        })
    }

    fn room_matches_type_behind(
        spots: &Vec<Occupant>,
        room: &Room,
        pod_type: PodType,
        pos: usize,
    ) -> bool {
        match room.positions.iter().position(|v| *v == pos) {
            // Pos isn't in room.
            None => false,
            // Verify everything behind it matches it.
            Some(room_idx) => room
                .positions
                .iter()
                .skip(room_idx)
                .all(|v| spots[*v] == Occupant::Pod(pod_type)),
        }
    }

    fn farthest_back_open_spot(&self, room: &Room) -> Option<usize> {
        room.positions
            .iter()
            .take_while(|idx| self.spots[**idx] == Occupant::Empty)
            .last()
            .cloned()
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
        for i in 0..4 {
            let start = 7 + (i * 4);
            let extra = if i == 0 {
                format!("    cost: {}", self.cost_so_far)
            } else if i == 1 {
                format!("    rem:  {}", self.estimated_remaining_cost)
            } else {
                String::new()
            };
            println!(
                "  {:?} {:?} {:?} {:?}{}",
                self.spots[start + 0],
                self.spots[start + 1],
                self.spots[start + 2],
                self.spots[start + 3],
                extra
            );
        }
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
            // board.print_ancestry();
            return board.cost_so_far;
        }

        frontier.extend(board.possible_moves());
        seen_spots.insert(board.spots);
    }
    panic!("Unable to find sort for board");
}

pub fn main() {
    // let input = "BCBDADCA";
    let input = "DBACCADB";

    let padding_text = "ABCDABCD";
    let part1_initial_board = Board::new((format!("{}{}", input, padding_text)).as_str());
    // part1_initial_board.print();
    // 11489

    // 12521
    // 15538
    println!("Part 1: {}", search_cheapest_sort_path(part1_initial_board));

    let extra_text = "DCBADBAC";
    let (first, second) = input.split_at(4);
    let part2_initial_board = Board::new((format!("{}{}{}", first, extra_text, second)).as_str());
    println!("Part 2: {}", search_cheapest_sort_path(part2_initial_board));
}
