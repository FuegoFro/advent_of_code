use std::ops;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Delta3 {
    pub dx: i32,
    pub dy: i32,
    pub dz: i32,
}

impl Delta3 {
    pub const IDENT: Delta3 = Delta3::new(0, 0, 0);
    pub const X_POS: Delta3 = Delta3::new(1, 0, 0);
    pub const X_NEG: Delta3 = Delta3::new(-1, 0, 0);
    pub const Y_POS: Delta3 = Delta3::new(0, 1, 0);
    pub const Y_NEG: Delta3 = Delta3::new(0, -1, 0);
    pub const Z_POS: Delta3 = Delta3::new(0, 0, 1);
    pub const Z_NEG: Delta3 = Delta3::new(0, 0, -1);

    pub const fn new(dx: i32, dy: i32, dz: i32) -> Self {
        Delta3 { dx, dy, dz }
    }

    pub fn l1_dist(&self) -> i32 {
        self.dx.abs() + self.dy.abs() + self.dz.abs()
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    pub const ORIGIN: Point3 = Point3::new(0, 0, 0);

    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Point3 { x, y, z }
    }
}

impl_op_ex!(+ |a: &Point3, b: &Delta3| -> Point3 { Point3 { x: a.x + b.dx, y: a.y + b.dy, z: a.z + b.dz }});
impl_op_ex!(-|a: &Point3, b: &Delta3| -> Point3 {
    Point3 {
        x: a.x - b.dx,
        y: a.y - b.dy,
        z: a.z - b.dz,
    }
});
impl_op_ex!(-|a: &Point3, b: &Point3| -> Delta3 {
    Delta3 {
        dx: a.x - b.x,
        dy: a.y - b.y,
        dz: a.z - b.z,
    }
});
impl_op_ex!(*|a: &Point3, b: i32| -> Point3 {
    Point3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});
impl_op_ex!(/|a: &Point3, b: i32| -> Point3 {
    Point3 {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    }
});
impl_op!(+= |a: &mut Point3, b: Delta3| { *a = &*a + b });
impl_op!(+= |a: &mut Point3, b: &Delta3| { *a = &*a + b });
impl_op!(-= |a: &mut Point3, b: Delta3| { *a = &*a - b });
impl_op!(-= |a: &mut Point3, b: &Delta3| { *a = &*a - b });
