use crate::util::min_max;
use std::ops;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Delta {
    pub dx: i32,
    pub dy: i32,
}

impl Delta {
    pub const UP_LEFT: Delta = Delta::new(-1, -1);
    pub const UP: Delta = Delta::new(0, -1);
    pub const UP_RIGHT: Delta = Delta::new(1, -1);
    pub const LEFT: Delta = Delta::new(-1, 0);
    pub const NONE: Delta = Delta::new(0, 0);
    pub const RIGHT: Delta = Delta::new(1, 0);
    pub const DOWN_LEFT: Delta = Delta::new(-1, 1);
    pub const DOWN: Delta = Delta::new(0, 1);
    pub const DOWN_RIGHT: Delta = Delta::new(1, 1);

    pub const NEIGHBORS4: [Delta; 4] = [Delta::UP, Delta::LEFT, Delta::RIGHT, Delta::DOWN];
    pub const NEIGHBORS8: [Delta; 8] = [
        Delta::UP_LEFT,
        Delta::UP,
        Delta::UP_RIGHT,
        Delta::LEFT,
        Delta::RIGHT,
        Delta::DOWN_LEFT,
        Delta::DOWN,
        Delta::DOWN_RIGHT,
    ];
    pub const NEIGHBORS9: [Delta; 9] = [
        Delta::UP_LEFT,
        Delta::UP,
        Delta::UP_RIGHT,
        Delta::LEFT,
        Delta::NONE,
        Delta::RIGHT,
        Delta::DOWN_LEFT,
        Delta::DOWN,
        Delta::DOWN_RIGHT,
    ];

    pub const fn new(dx: i32, dy: i32) -> Self {
        Delta { dx, dy }
    }
}

impl_op_ex!(+|a: &Delta, b: &Delta| -> Delta {
    Delta {
        dx: a.dx + b.dx,
        dy: a.dy + b.dy,
    }
});
impl_op_ex!(-|a: &Delta, b: &Delta| -> Delta {
    Delta {
        dx: a.dx - b.dx,
        dy: a.dy - b.dy,
    }
});
impl_op_ex!(*|a: &Delta, b: i32| -> Delta {
    Delta {
        dx: a.dx * b,
        dy: a.dy * b,
    }
});
impl_op_ex!(/|a: &Delta, b: i32| -> Delta {
    Delta {
        dx: a.dx / b,
        dy: a.dy / b,
    }
});
impl_op!(+= |a: &mut Delta, b: Delta| { *a = *a + b });
impl_op!(+= |a: &mut Delta, b: &Delta| { *a = *a + b });
impl_op!(-= |a: &mut Delta, b: Delta| { *a = *a - b });
impl_op!(-= |a: &mut Delta, b: &Delta| { *a = *a - b });

// TODO - Make these points generic to reduce copy/paste.
//  Requires https://github.com/carbotaniuman/auto_ops/pull/4

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct PointS {
    pub x: i32,
    pub y: i32,
}

impl PointS {
    pub const ORIGIN: PointS = PointS { x: 0, y: 0 };

    pub const fn new(x: i32, y: i32) -> PointS {
        PointS { x, y }
    }

    pub fn get_bounding_box<'a>(points: impl Iterator<Item = &'a PointS>) -> (PointS, PointS) {
        let (xs, ys): (Vec<_>, Vec<_>) = points.map(|p| (p.x, p.y)).unzip();
        let (min_x, max_x) = min_max(xs);
        let (min_y, may_y) = min_max(ys);
        (PointS::new(min_x, min_y), PointS::new(max_x, may_y))
    }
}

impl_op_ex!(+ |a: &PointS, b: &Delta| -> PointS { PointS { x: a.x + b.dx, y: a.y + b.dy }});
impl_op_ex!(-|a: &PointS, b: &Delta| -> PointS {
    PointS {
        x: a.x - b.dx,
        y: a.y - b.dy,
    }
});
impl_op_ex!(-|a: &PointS, b: &PointS| -> Delta {
    Delta {
        dx: a.x - b.x,
        dy: a.y - b.y,
    }
});
impl_op_ex!(*|a: &PointS, b: i32| -> PointS {
    PointS {
        x: a.x * b,
        y: a.y * b,
    }
});
impl_op_ex!(/|a: &PointS, b: i32| -> PointS {
    PointS {
        x: a.x / b,
        y: a.y / b,
    }
});
impl_op!(+= |a: &mut PointS, b: Delta| { *a = *a + b });
impl_op!(+= |a: &mut PointS, b: &Delta| { *a = *a + b });
impl_op!(-= |a: &mut PointS, b: Delta| { *a = *a - b });
impl_op!(-= |a: &mut PointS, b: &Delta| { *a = *a - b });

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct PointU {
    pub x: usize,
    pub y: usize,
}

impl PointU {
    pub const ORIGIN: PointU = PointU { x: 0, y: 0 };

    pub const fn new(x: usize, y: usize) -> PointU {
        PointU { x, y }
    }

    pub fn checked_add(&self, delta: &Delta) -> Option<Self> {
        if let (Some(x), Some(y)) = (
            checked_add_unsigned(self.x, delta.dx),
            checked_add_unsigned(self.y, delta.dy),
        ) {
            Some(Self { x, y })
        } else {
            None
        }
    }
}

fn checked_add_unsigned(a: usize, b: i32) -> Option<usize> {
    if b >= 0 {
        a.checked_add(b as usize)
    } else {
        a.checked_sub((-b) as usize)
    }
}

fn checked_sub_unsigned(a: usize, b: i32) -> Option<usize> {
    if b >= 0 {
        a.checked_sub(b as usize)
    } else {
        a.checked_add((-b) as usize)
    }
}

fn add_unsigned(a: usize, b: i32) -> usize {
    if b >= 0 {
        a + b as usize
    } else {
        a - (-b) as usize
    }
}

fn sub_unsigned(a: usize, b: i32) -> usize {
    if b >= 0 {
        a - b as usize
    } else {
        a + (-b) as usize
    }
}

impl_op_ex!(+|a: &PointU, b: &Delta| -> PointU {
    PointU {
        x: add_unsigned(a.x, b.dx),
        y: add_unsigned(a.y, b.dy),
    }
});
impl_op_ex!(-|a: &PointU, b: &Delta| -> PointU {
    PointU {
        x: sub_unsigned(a.x, b.dx),
        y: sub_unsigned(a.y, b.dy),
    }
});
impl_op_ex!(-|a: &PointU, b: &PointU| -> Delta {
    Delta {
        dx: a.x as i32 - b.x as i32,
        dy: a.y as i32 - b.y as i32,
    }
});
impl_op_ex!(*|a: &PointU, b: usize| -> PointU {
    PointU {
        x: a.x * b,
        y: a.y * b,
    }
});
impl_op_ex!(/|a: &PointU, b: usize| -> PointU {
    PointU {
        x: a.x / b,
        y: a.y / b,
    }
});
impl_op!(+= |a: &mut PointU, b: Delta| { *a = *a + b });
impl_op!(+= |a: &mut PointU, b: &Delta| { *a = *a + b });
impl_op!(-= |a: &mut PointU, b: Delta| { *a = *a - b });
impl_op!(-= |a: &mut PointU, b: &Delta| { *a = *a - b });
