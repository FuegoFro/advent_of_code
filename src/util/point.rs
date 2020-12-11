use std::ops;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl_op_ex!(+ |a: &Point, b: &Point| -> Point { Point { x: a.x + b.x, y: a.y + b.y }});
impl_op!(+= |a: &mut Point, b: Point| { *a = &*a + b });
impl_op!(+= |a: &mut Point, b: &Point| { *a = &*a + b });

impl Point {
    pub fn l1_dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
