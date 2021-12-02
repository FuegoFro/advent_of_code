use itertools::{Itertools, MinMaxResult};
use std::ops;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl_op_ex!(+ |a: &Point, b: &Point| -> Point { Point { x: a.x + b.x, y: a.y + b.y }});
impl_op_ex!(-|a: &Point, b: &Point| -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});
impl_op_ex!(*|a: &Point, b: i32| -> Point {
    Point {
        x: a.x * b,
        y: a.y * b,
    }
});
impl_op_ex!(/|a: &Point, b: i32| -> Point {
    Point {
        x: a.x / b,
        y: a.y / b,
    }
});
impl_op_ex!(*|a: &Point, b: u32| -> Point {
    Point {
        x: a.x * b as i32,
        y: a.y * b as i32,
    }
});
impl_op!(+= |a: &mut Point, b: Point| { *a = &*a + b });
impl_op!(+= |a: &mut Point, b: &Point| { *a = &*a + b });
impl_op!(-= |a: &mut Point, b: Point| { *a = &*a - b });
impl_op!(-= |a: &mut Point, b: &Point| { *a = &*a - b });

impl Point {
    pub const ORIGIN: Point = Point { x: 0, y: 0 };
    pub const UP: Point = Point { x: 0, y: 1 };
    pub const DOWN: Point = Point { x: 0, y: -1 };
    pub const LEFT: Point = Point { x: -1, y: 0 };
    pub const RIGHT: Point = Point { x: 1, y: 0 };

    pub const fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn l1_dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    /// Returns a new point that has been rotate counter clockwise by the given degrees.
    pub fn rotate_about_origin_deg(&self, deg: u32) -> Point {
        let deg_mod = ((deg % 360) + 360) % 360;
        let (sin, cos) = match deg_mod {
            0 => (0, 1),
            90 => (1, 0),
            180 => (0, -1),
            270 => (-1, 0),
            _ => panic!("Can only handle multiples of 90 degrees, got {}", deg),
        };
        Point {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    pub fn atan2(&self) -> f64 {
        (self.y as f64).atan2(self.x as f64)
    }
}

pub fn get_bounding_box(points: Vec<&Point>) -> (Point, Point) {
    let (min_x, max_x) = match points.iter().map(|p| p.x).minmax() {
        MinMaxResult::NoElements => panic!("Expected some elements"),
        MinMaxResult::OneElement(e) => (e, e),
        MinMaxResult::MinMax(l, h) => (l, h),
    };
    let (min_y, may_y) = match points.iter().map(|p| p.y).minmax() {
        MinMaxResult::NoElements => panic!("Expected some elements"),
        MinMaxResult::OneElement(e) => (e, e),
        MinMaxResult::MinMax(l, h) => (l, h),
    };
    (Point::new(min_x, min_y), Point::new(max_x, may_y))
}
