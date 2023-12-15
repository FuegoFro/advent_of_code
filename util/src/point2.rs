use num_traits::{Num, NumCast, Signed};

use crate::additional_num_traits::{CheckedOps, NegOneConst, ZeroOneConst};
use crate::min_max;

pub trait DeltaValue: Num + NumCast + ZeroOneConst + NegOneConst + Copy {}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Delta<DV: DeltaValue> {
    pub dx: DV,
    pub dy: DV,
}

impl<DV: DeltaValue> Delta<DV> {
    pub const UP_LEFT: Self = Self::new(DV::NEG_ONE, DV::NEG_ONE);
    pub const UP: Self = Self::new(DV::ZERO, DV::NEG_ONE);
    pub const UP_RIGHT: Self = Self::new(DV::ONE, DV::NEG_ONE);
    pub const LEFT: Self = Self::new(DV::NEG_ONE, DV::ZERO);
    pub const NONE: Self = Self::new(DV::ZERO, DV::ZERO);
    pub const RIGHT: Self = Self::new(DV::ONE, DV::ZERO);
    pub const DOWN_LEFT: Self = Self::new(DV::NEG_ONE, DV::ONE);
    pub const DOWN: Self = Self::new(DV::ZERO, DV::ONE);
    pub const DOWN_RIGHT: Self = Self::new(DV::ONE, DV::ONE);

    pub const NEIGHBORS4: [Self; 4] = [Self::UP, Self::LEFT, Self::RIGHT, Self::DOWN];
    pub const NEIGHBORS8: [Self; 8] = [
        Self::UP_LEFT,
        Self::UP,
        Self::UP_RIGHT,
        Self::LEFT,
        Self::RIGHT,
        Self::DOWN_LEFT,
        Self::DOWN,
        Self::DOWN_RIGHT,
    ];
    pub const NEIGHBORS9: [Self; 9] = [
        Self::UP_LEFT,
        Self::UP,
        Self::UP_RIGHT,
        Self::LEFT,
        Self::NONE,
        Self::RIGHT,
        Self::DOWN_LEFT,
        Self::DOWN,
        Self::DOWN_RIGHT,
    ];
    pub const DIAGONALS: [Self; 4] = [
        Self::UP_LEFT,
        Self::UP_RIGHT,
        Self::DOWN_LEFT,
        Self::DOWN_RIGHT,
    ];

    pub const fn new(dx: DV, dy: DV) -> Self {
        Self { dx, dy }
    }

    pub fn rotate_about_origin_deg(&self, rotation: Rotation) -> Self {
        let (sin, cos) = match rotation {
            Rotation::Deg0 => (DV::ZERO, DV::ONE),
            Rotation::Deg90 => (DV::ONE, DV::ZERO),
            Rotation::Deg180 => (DV::ZERO, DV::NEG_ONE),
            Rotation::Deg270 => (DV::NEG_ONE, DV::ZERO),
        };
        let dx = self.dx * cos - self.dy * sin;
        let dy = self.dx * sin + self.dy * cos;
        Self::new(dx, dy)
    }

    pub fn l1_dist(&self) -> DV {
        self.dx.abs() + self.dy.abs()
    }

    pub fn unit(&self) -> Self {
        self / self.l1_dist()
    }

    pub fn area(&self) -> DV {
        self.dx.abs() * self.dy.abs()
    }

    pub fn cast<Out: DeltaValue>(&self) -> Option<Delta<Out>> {
        match (num_traits::cast(self.dx), num_traits::cast(self.dy)) {
            (Some(dx), Some(dy)) => Some(Delta::new(dx, dy)),
            _ => None,
        }
    }
}

pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl_op_ex!(+ <DV: DeltaValue> |a: &Delta<DV>, b: &Delta<DV>| -> Delta<DV> {
    Delta {
        dx: a.dx + b.dx,
        dy: a.dy + b.dy,
    }
});
impl_op_ex!(- <DV: DeltaValue> |a: &Delta<DV>, b: &Delta<DV>| -> Delta<DV> {
    Delta {
        dx: a.dx - b.dx,
        dy: a.dy - b.dy,
    }
});
// Can't make this commutative since we can't implement the foreign Mul trait on an arbitrary DV :(
impl_op_ex!(* <DV: DeltaValue> |a: &Delta<DV>, b: DV| -> Delta<DV> {
    Delta {
        dx: a.dx * b,
        dy: a.dy * b,
    }
});
impl_op_ex!(/ <DV: DeltaValue> |a: &Delta<DV>, b: DV| -> Delta<DV> {
    Delta {
        dx: a.dx / b,
        dy: a.dy / b,
    }
});
impl_op_ex!(+= <DV: DeltaValue> |a: &mut Delta<DV>, b: &Delta<DV>| { *a = *a + b });
impl_op_ex!(-= <DV: DeltaValue> |a: &mut Delta<DV>, b: &Delta<DV>| { *a = *a - b });

pub trait PointValue:
    Num + NumCast + ZeroOneConst + CheckedOps + Copy + PartialOrd + 'static
{
    type DeltaValueType: DeltaValue;
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Point<PV: PointValue> {
    pub x: PV,
    pub y: PV,
}

impl<PV: PointValue> Point<PV> {
    pub const ORIGIN: Self = Self::new(PV::ZERO, PV::ZERO);

    pub const fn new(x: PV, y: PV) -> Self {
        Self { x, y }
    }

    pub fn cast<Out: PointValue>(&self) -> Option<Point<Out>> {
        match (num_traits::cast(self.x), num_traits::cast(self.y)) {
            (Some(x), Some(y)) => Some(Point::new(x, y)),
            _ => None,
        }
    }

    pub fn get_bounding_box<'a>(points: impl Iterator<Item = &'a Self>) -> (Self, Self) {
        let (xs, ys): (Vec<_>, Vec<_>) = points.map(|p| (p.x, p.y)).unzip();
        let (min_x, max_x) = min_max(xs);
        let (min_y, may_y) = min_max(ys);
        (Self::new(min_x, min_y), Self::new(max_x, may_y))
    }

    pub fn checked_add(&self, delta: &Delta<PV::DeltaValueType>) -> Option<Self> {
        match (
            do_op(self.x, delta.dx, Op::Add),
            do_op(self.y, delta.dy, Op::Add),
        ) {
            (Some(x), Some(y)) => Some(Self { x, y }),
            _ => None,
        }
    }
}

enum Op {
    Add,
    Sub,
}

impl Op {
    fn neg(&self) -> Self {
        match self {
            Op::Add => Op::Sub,
            Op::Sub => Op::Add,
        }
    }
}

fn do_op<SelfT: CheckedOps + NumCast, OtherT: Signed + NumCast>(
    lhs: SelfT,
    mut rhs: OtherT,
    mut op: Op,
) -> Option<SelfT> {
    if rhs.is_negative() {
        rhs = rhs.neg();
        op = op.neg();
    }
    // We always want to cast a non-negative value, in case we're casting to an unsigned type
    let rhs = num_traits::cast(rhs)?;
    match op {
        Op::Add => lhs.checked_add(&rhs),
        Op::Sub => lhs.checked_sub(&rhs),
    }
}

impl_op_ex_commutative!(
    +
    <PV: PointValue<DeltaValueType=DV>, DV: DeltaValue>
    |a: &Point<PV>, b: &Delta<DV>| -> Point<PV> {
        a.checked_add(b).unwrap()
    }
);
impl_op_ex!(- <PV: PointValue, DV: DeltaValue> |a: &Point<PV>, b: &Delta<DV>| -> Point<PV> {
    Point {
        x: do_op(a.x, b.dx, Op::Sub).unwrap(),
        y: do_op(a.y, b.dy, Op::Sub).unwrap(),
    }
});
impl_op_ex!(- <PV: PointValue> |a: &Point<PV>, b: &Point<PV>| -> Delta<PV::DeltaValueType> {
    Delta {
        // Cast first to ensure we're using signed types, then do the subtraction.
        dx: num_traits::cast::<PV, PV::DeltaValueType>(a.x).unwrap() - num_traits::cast(b.x).unwrap(),
        dy: num_traits::cast::<PV, PV::DeltaValueType>(a.y).unwrap() - num_traits::cast(b.y).unwrap(),
    }
});
// Can't make this commutative since we can't implement the foreign Mul trait on an arbitrary PV :(
impl_op_ex!(* <PV: PointValue> |a: &Point<PV>, b: &PV| -> Point<PV> {
    Point {
        x: a.x * *b,
        y: a.y * *b,
    }
});
impl_op_ex!(/ <PV: PointValue> |a: &Point<PV>, b: &PV| -> Point<PV> {
    Point {
        x: a.x / *b,
        y: a.y / *b,
    }
});
impl_op_ex!(+= <PV: PointValue> |a: &mut Point<PV>, b: &Delta<PV::DeltaValueType>| { *a = *a + b });
impl_op_ex!(-= <PV: PointValue> |a: &mut Point<PV>, b: &Delta<PV::DeltaValueType>| { *a = *a - b });

/*
 * Impl all the traits!
 */

// Impl the Delta traits
impl DeltaValue for isize {}
impl DeltaValue for i8 {}
impl DeltaValue for i16 {}
impl DeltaValue for i32 {}
impl DeltaValue for i64 {}
impl DeltaValue for i128 {}
impl DeltaValue for f32 {}
impl DeltaValue for f64 {}

// Impl the Point traits
macro_rules! impl_point_value {
    ($delta_value_type:ty; $($point_value_type:ty),*) => {
        $(
            impl PointValue for $point_value_type {
                type DeltaValueType = $delta_value_type;
            }
        )*
    };
}

impl_point_value!(isize; isize, usize);
impl_point_value!(i8; i8, u8);
impl_point_value!(i16; i16, u16);
impl_point_value!(i32; i32, u32);
impl_point_value!(i64; i64, u64);
impl_point_value!(i128; i128, u128);
impl_point_value!(f32; f32);
impl_point_value!(f64; f64);

// Convenience aliases
pub type DeltaS = Delta<i32>;
pub type PointS = Point<i32>;
pub type PointU = Point<usize>;
