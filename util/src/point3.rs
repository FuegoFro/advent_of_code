use std::fmt::Debug;
use std::hash::Hash;

use itertools::Itertools;
use num_traits::{Num, NumCast};

use crate::additional_num_traits::{NegOneConst, ZeroOneConst};
use crate::min_max;

pub trait PointValue:
    Num
    + NumCast
    + ZeroOneConst
    + NegOneConst
    + Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Debug
    + Hash
    + 'static
{
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Delta3G<DV: PointValue> {
    pub dx: DV,
    pub dy: DV,
    pub dz: DV,
}

impl<DV: PointValue> Delta3G<DV> {
    pub const IDENT: Self = Self::new(DV::ZERO, DV::ZERO, DV::ZERO);
    pub const X_POS: Self = Self::new(DV::ONE, DV::ZERO, DV::ZERO);
    pub const X_NEG: Self = Self::new(DV::NEG_ONE, DV::ZERO, DV::ZERO);
    pub const Y_POS: Self = Self::new(DV::ZERO, DV::ONE, DV::ZERO);
    pub const Y_NEG: Self = Self::new(DV::ZERO, DV::NEG_ONE, DV::ZERO);
    pub const Z_POS: Self = Self::new(DV::ZERO, DV::ZERO, DV::ONE);
    pub const Z_NEG: Self = Self::new(DV::ZERO, DV::ZERO, DV::NEG_ONE);

    pub const NEIGHBORS_6: [Self; 6] = [
        Self::X_POS,
        Self::X_NEG,
        Self::Y_POS,
        Self::Y_NEG,
        Self::Z_POS,
        Self::Z_NEG,
    ];

    pub const fn new(dx: DV, dy: DV, dz: DV) -> Self {
        Self { dx, dy, dz }
    }

    pub fn l1_dist(&self) -> DV {
        self.dx.abs() + self.dy.abs() + self.dz.abs()
    }
}

impl_op_ex!(+ <DV: PointValue> |a: &Delta3G<DV>, b: &Delta3G<DV>| -> Delta3G<DV> { Delta3G { dx: a.dx + b.dx, dy: a.dy + b.dy, dz: a.dz + b.dz }});
impl_op_ex!(- <DV: PointValue> |a: &Delta3G<DV>, b: &Delta3G<DV>| -> Delta3G<DV> {
    Delta3G {
        dx: a.dx - b.dx,
        dy: a.dy - b.dy,
        dz: a.dz - b.dz,
    }
});
impl_op_ex!(* <DV: PointValue> |a: &Delta3G<DV>, b: DV| -> Delta3G<DV> {
    Delta3G {
        dx: a.dx * b,
        dy: a.dy * b,
        dz: a.dz * b,
    }
});
impl_op_ex!(/ <DV: PointValue> |a: &Delta3G<DV>, b: DV| -> Delta3G<DV> {
    Delta3G {
        dx: a.dx / b,
        dy: a.dy / b,
        dz: a.dz / b,
    }
});
impl_op!(+= <DV: PointValue> |a: &mut Delta3G<DV>, b: Delta3G<DV>| { *a = *a + b });
impl_op!(+= <DV: PointValue> |a: &mut Delta3G<DV>, b: &Delta3G<DV>| { *a = *a + b });
impl_op!(-= <DV: PointValue> |a: &mut Delta3G<DV>, b: Delta3G<DV>| { *a = *a - b });
impl_op!(-= <DV: PointValue> |a: &mut Delta3G<DV>, b: &Delta3G<DV>| { *a = *a - b });

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point3G<PV: PointValue> {
    pub x: PV,
    pub y: PV,
    pub z: PV,
}

impl<PV: PointValue> Point3G<PV> {
    pub const ORIGIN: Self = Self::new(PV::ZERO, PV::ZERO, PV::ZERO);

    pub const fn new(x: PV, y: PV, z: PV) -> Self {
        Self { x, y, z }
    }

    pub fn clamp(&self, bound: &BoundingBoxG<PV>) -> Self {
        Self::new(
            clamp(self.x, bound.start.x, bound.end.x),
            clamp(self.y, bound.start.y, bound.end.y),
            clamp(self.z, bound.start.z, bound.end.z),
        )
    }

    pub fn get_bounding_box<'a>(points: impl Iterator<Item = &'a Self>) -> BoundingBoxG<PV> {
        BoundingBoxG::containing_points(points)
    }

    pub fn cast<Out: PointValue>(&self) -> Option<Point3G<Out>> {
        match (
            num_traits::cast(self.x),
            num_traits::cast(self.y),
            num_traits::cast(self.z),
        ) {
            (Some(x), Some(y), Some(z)) => Some(Point3G::new(x, y, z)),
            _ => None,
        }
    }
}

fn clamp<PV: PointValue>(val: PV, min: PV, max: PV) -> PV {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

fn max<PV: PointValue>(a: PV, b: PV) -> PV {
    if a > b {
        a
    } else {
        b
    }
}

fn min<PV: PointValue>(a: PV, b: PV) -> PV {
    if a < b {
        a
    } else {
        b
    }
}

impl_op_ex!(+ <PV: PointValue> |a: &Point3G<PV>, b: &Delta3G<PV>| -> Point3G<PV> { Point3G { x: a.x + b.dx, y: a.y + b.dy, z: a.z + b.dz }});
impl_op_ex!(- <PV: PointValue> |a: &Point3G<PV>, b: &Delta3G<PV>| -> Point3G<PV> {
    Point3G {
        x: a.x - b.dx,
        y: a.y - b.dy,
        z: a.z - b.dz,
    }
});
impl_op_ex!(- <PV: PointValue> |a: &Point3G<PV>, b: &Point3G<PV>| -> Delta3G<PV> {
    Delta3G {
        dx: a.x - b.x,
        dy: a.y - b.y,
        dz: a.z - b.z,
    }
});
impl_op_ex!(* <PV:PointValue> |a: &Point3G<PV>, b: PV| -> Point3G<PV> {
    Point3G {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});
impl_op_ex!(/ <PV: PointValue> |a: &Point3G<PV>, b: PV| -> Point3G<PV> {
    Point3G {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    }
});
impl_op!(+= <PV: PointValue> |a: &mut Point3G<PV>, b: Delta3G<PV>| { *a = *a + b });
impl_op!(+= <PV: PointValue> |a: &mut Point3G<PV>, b: &Delta3G<PV>| { *a = *a + b });
impl_op!(-= <PV: PointValue> |a: &mut Point3G<PV>, b: Delta3G<PV>| { *a = *a - b });
impl_op!(-= <PV: PointValue> |a: &mut Point3G<PV>, b: &Delta3G<PV>| { *a = *a - b });

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BoundingBoxG<PV: PointValue> {
    // Inclusive
    pub start: Point3G<PV>,
    // Exclusive
    pub end: Point3G<PV>,
}

#[derive(Debug)]
pub enum OverlapResultG<PV: PointValue> {
    // Bounds are the same
    Identical,
    // Left is fully within (strict subset of) right
    FullyContainedByArg,
    // Left fully contains (strict superset of) right
    FullyContainsArg,
    PartialOverlap {
        // Don't want to use BoundingBox since we don't know that the arg is closer to the start,
        // want to be able to distinguish which is self vs arg.
        self_mid_point: Point3G<PV>,
        arg_mid_point: Point3G<PV>,
    },
    NoOverlap,
}

impl<PV: PointValue> BoundingBoxG<PV> {
    pub const EMPTY: Self = Self::new(Point3G::ORIGIN, Point3G::ORIGIN);

    pub const fn new(start: Point3G<PV>, end: Point3G<PV>) -> Self {
        Self { start, end }
    }

    pub fn containing_points<'a>(points: impl Iterator<Item = &'a Point3G<PV>>) -> Self {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        let mut zs = Vec::new();
        for p in points {
            xs.push(p.x);
            ys.push(p.y);
            zs.push(p.z);
        }
        let (min_x, max_x) = min_max(xs);
        let (min_y, max_y) = min_max(ys);
        let (min_z, max_z) = min_max(zs);
        BoundingBoxG::new(
            Point3G::new(min_x, min_y, min_z),
            Point3G::new(max_x, max_y, max_z),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn corners(&self) -> [Point3G<PV>; 8] {
        let xs = [self.start.x, self.end.x];
        let ys = [self.start.y, self.end.y];
        let zs = [self.start.z, self.end.z];
        let mut points = Vec::new();
        for z in zs.into_iter() {
            for y in ys.into_iter() {
                for x in xs.into_iter() {
                    points.push(Point3G::new(x, y, z));
                }
            }
        }
        points.try_into().unwrap()
    }

    pub fn octants(&self, mid: &Point3G<PV>) -> [Self; 8] {
        let xs = [self.start.x, mid.x, self.end.x];
        let ys = [self.start.y, mid.y, self.end.y];
        let zs = [self.start.z, mid.z, self.end.z];
        let mut points = Vec::new();
        for (z_start, z_end) in zs.into_iter().tuple_windows() {
            for (y_start, y_end) in ys.into_iter().tuple_windows() {
                for (x_start, x_end) in xs.into_iter().tuple_windows() {
                    points.push(BoundingBoxG::new(
                        Point3G::new(x_start, y_start, z_start),
                        Point3G::new(x_end, y_end, z_end),
                    ));
                }
            }
        }
        points.try_into().unwrap()
    }

    pub fn contains(&self, point: &Point3G<PV>) -> bool {
        (self.start.x <= point.x && point.x < self.end.x)
            && (self.start.y <= point.y && point.y < self.end.y)
            && (self.start.z <= point.z && point.z < self.end.z)
    }

    pub fn intersect(&self, other: &Self) -> Self {
        let start = Point3G::new(
            max(self.start.x, other.start.x),
            max(self.start.y, other.start.y),
            max(self.start.z, other.start.z),
        );
        let end = Point3G::new(
            max(min(self.end.x, other.end.x), start.x),
            max(min(self.end.y, other.end.y), start.y),
            max(min(self.end.z, other.end.z), start.z),
        );
        Self::new(start, end)
    }

    pub fn union(&self, other: &Self) -> Self {
        Self::containing_points([self.start, self.end, other.start, other.end].iter())
    }

    pub fn get_overlap(&self, other: &Self) -> OverlapResultG<PV> {
        // eprintln!("self = {:?}", self);
        // eprintln!("other = {:?}", other);
        let union = self.union(other);
        // eprintln!("union = {:?}", union);
        let intersection = self.intersect(other);
        if self == other {
            OverlapResultG::Identical
        } else if union == *other {
            OverlapResultG::FullyContainedByArg
        } else if union == *self {
            OverlapResultG::FullyContainsArg
        } else if !intersection.is_empty() {
            // Calculate the best mid-points for each.
            OverlapResultG::PartialOverlap {
                self_mid_point: self.get_best_mid_point(&intersection),
                arg_mid_point: other.get_best_mid_point(&intersection),
            }
        } else {
            OverlapResultG::NoOverlap
        }
    }

    pub fn cast<Out: PointValue>(&self) -> Option<BoundingBoxG<Out>> {
        match (self.start.cast(), self.end.cast()) {
            (Some(start), Some(end)) => Some(BoundingBoxG::new(start, end)),
            _ => None,
        }
    }

    pub fn volume(&self) -> PV {
        let delta = self.end - self.start;
        delta.dx * delta.dy * delta.dz
    }

    pub fn get_best_mid_point(&self, mid_point_options: &Self) -> Point3G<PV> {
        let points_by_score = mid_point_options
            .corners()
            .into_iter()
            .map(|p| (self.score_mid_point(&p), p))
            .into_group_map();
        let max_score = points_by_score.keys().max().unwrap();
        points_by_score.get(max_score).unwrap()[0]
    }

    fn score_mid_point(&self, point: &Point3G<PV>) -> PV {
        min(point.x - self.start.x, self.end.x - point.x)
            + min(point.y - self.start.y, self.end.y - point.y)
            + min(point.z - self.start.z, self.end.z - point.z)
    }
}

// Impl the Delta traits
impl PointValue for isize {}
impl PointValue for i8 {}
impl PointValue for i16 {}
impl PointValue for i32 {}
impl PointValue for i64 {}
impl PointValue for i128 {}

pub type Delta3 = Delta3G<i32>;
pub type Point3 = Point3G<i32>;
pub type BoundingBox = BoundingBoxG<i32>;
pub type OverlapResult = OverlapResultG<i32>;

#[cfg(test)]
mod test {
    use crate::point3::{BoundingBox, OverlapResult, Point3};

    #[test]
    fn test_containing_points() {
        assert_eq!(
            BoundingBox::containing_points(
                [
                    Point3::new(1, 0, 0),
                    Point3::new(0, 2, 0),
                    Point3::new(0, 0, 3),
                    Point3::new(-1, 0, 0),
                    Point3::new(0, -2, 0),
                    Point3::new(0, 0, -3),
                    Point3::new(0, 0, 0),
                ]
                .iter()
            ),
            BoundingBox::new(Point3::new(-1, -2, -3), Point3::new(1, 2, 3))
        );
    }

    #[test]
    fn test_octants() {
        let bound = BoundingBox::new(Point3::ORIGIN, Point3::new(4, 4, 4));
        assert_eq!(
            bound.octants(&Point3::new(2, 2, 2)),
            [
                BoundingBox::new(Point3::new(0, 0, 0), Point3::new(2, 2, 2)),
                BoundingBox::new(Point3::new(2, 0, 0), Point3::new(4, 2, 2)),
                BoundingBox::new(Point3::new(0, 2, 0), Point3::new(2, 4, 2)),
                BoundingBox::new(Point3::new(2, 2, 0), Point3::new(4, 4, 2)),
                BoundingBox::new(Point3::new(0, 0, 2), Point3::new(2, 2, 4)),
                BoundingBox::new(Point3::new(2, 0, 2), Point3::new(4, 2, 4)),
                BoundingBox::new(Point3::new(0, 2, 2), Point3::new(2, 4, 4)),
                BoundingBox::new(Point3::new(2, 2, 2), Point3::new(4, 4, 4)),
            ]
        )
    }

    #[test]
    fn test_corners() {
        let bound = BoundingBox::new(Point3::ORIGIN, Point3::new(4, 4, 4));
        assert_eq!(
            bound.corners(),
            [
                Point3::new(0, 0, 0),
                Point3::new(4, 0, 0),
                Point3::new(0, 4, 0),
                Point3::new(4, 4, 0),
                Point3::new(0, 0, 4),
                Point3::new(4, 0, 4),
                Point3::new(0, 4, 4),
                Point3::new(4, 4, 4),
            ]
        )
    }

    #[test]
    fn test_overlap_full_containment() {
        let outer = BoundingBox::new(Point3::new(0, 0, 0), Point3::new(4, 4, 4));
        let inner = BoundingBox::new(Point3::new(1, 1, 1), Point3::new(3, 3, 3));
        assert!(matches!(
            outer.get_overlap(&outer),
            OverlapResult::FullyContainedByArg
        ));
        assert!(matches!(
            inner.get_overlap(&outer),
            OverlapResult::FullyContainedByArg
        ));
        assert!(matches!(
            outer.get_overlap(&inner),
            OverlapResult::FullyContainsArg
        ));
    }

    #[test]
    fn test_overlap_no_overlap() {
        let start = Point3::new(0, 0, 0);
        let mid = Point3::new(2, 2, 2);
        let end = Point3::new(4, 4, 4);
        let a = BoundingBox::new(start, mid);
        let b = BoundingBox::new(mid, end);
        assert!(matches!(a.get_overlap(&b), OverlapResult::NoOverlap));
        assert!(matches!(b.get_overlap(&a), OverlapResult::NoOverlap));
    }
}
