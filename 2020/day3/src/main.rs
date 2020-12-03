use std::ops::Add;

struct Point {
    x: usize,
    y: usize,
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

struct TreeGrid {
    /// Vec of rows, each row being a vec of columns
    rows: Vec<Vec<bool>>,
}

impl TreeGrid {
    fn from_packed(packed: &str) -> Self {
        let rows = packed
            .split("\n")
            .map(|l| {
                // TODO - Better error handling for unexpected inputs
                l.chars().map(|c| c != '.').collect()
            })
            .collect();

        Self { rows }
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn has_tree_at(&self, point: &Point) -> bool {
        let row = &self.rows[point.y];
        let col = row[point.x % row.len()];

        col
    }
}

fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let slopes = vec![
        Point { x: 1, y: 1 },
        Point { x: 3, y: 1 },
        Point { x: 5, y: 1 },
        Point { x: 7, y: 1 },
        Point { x: 1, y: 2 },
    ];

    let result = slopes
        .iter()
        .map(|slope| {
            let grid = TreeGrid::from_packed(input);
            let mut current = Point { x: 0, y: 0 };
            let mut total: i64 = 0;
            while current.y < grid.height() {
                if grid.has_tree_at(&current) {
                    total += 1;
                }
                current = &current + &slope;
            }

            total
        })
        .fold(1, |acc, x| acc * x);

    println!("{}", result);
}
