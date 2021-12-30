use crate::util::min_max;
use itertools::Itertools;
use std::cmp::{max, min};
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

    pub fn clamp(&self, bound: &BoundingBox) -> Point3 {
        Point3::new(
            self.x.clamp(bound.start.x, bound.end.x),
            self.y.clamp(bound.start.y, bound.end.y),
            self.z.clamp(bound.start.z, bound.end.z),
        )
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BoundingBox {
    // Inclusive
    pub start: Point3,
    // Exclusive
    pub end: Point3,
}

#[derive(Debug)]
pub enum OverlapResult {
    // Bounds are the same
    Identical,
    // Left is fully within (strict subset of) right
    FullyContainedByArg,
    // Left fully contains (strict superset of) right
    FullyContainsArg,
    PartialOverlap {
        // Don't want to use BoundingBox since we don't know that the arg is closer to the start,
        // want to be able to distinguish which is self vs arg.
        self_mid_point: Point3,
        arg_mid_point: Point3,
    },
    NoOverlap,
}

impl BoundingBox {
    pub const EMPTY: BoundingBox = BoundingBox::new(Point3::ORIGIN, Point3::ORIGIN);

    pub const fn new(start: Point3, end: Point3) -> Self {
        BoundingBox { start, end }
    }

    pub fn containing_points<'a>(points: impl Iterator<Item = &'a Point3>) -> Self {
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
        BoundingBox::new(
            Point3::new(min_x, min_y, min_z),
            Point3::new(max_x, max_y, max_z),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.volume() == 0
    }

    pub fn corners(&self) -> [Point3; 8] {
        let xs = [self.start.x, self.end.x];
        let ys = [self.start.y, self.end.y];
        let zs = [self.start.z, self.end.z];
        let mut points = Vec::new();
        for z in zs.into_iter() {
            for y in ys.into_iter() {
                for x in xs.into_iter() {
                    points.push(Point3::new(x, y, z));
                }
            }
        }
        points.try_into().unwrap()
    }

    pub fn octants(&self, mid: &Point3) -> [BoundingBox; 8] {
        let xs = [self.start.x, mid.x, self.end.x];
        let ys = [self.start.y, mid.y, self.end.y];
        let zs = [self.start.z, mid.z, self.end.z];
        let mut points = Vec::new();
        for (z_start, z_end) in zs.into_iter().tuple_windows() {
            for (y_start, y_end) in ys.into_iter().tuple_windows() {
                for (x_start, x_end) in xs.into_iter().tuple_windows() {
                    points.push(BoundingBox::new(
                        Point3::new(x_start, y_start, z_start),
                        Point3::new(x_end, y_end, z_end),
                    ));
                }
            }
        }
        points.try_into().unwrap()
    }

    pub fn contains(&self, point: &Point3) -> bool {
        (self.start.x <= point.x && point.x < self.end.x)
            && (self.start.y <= point.y && point.y < self.end.y)
            && (self.start.z <= point.z && point.z < self.end.z)
    }

    pub fn intersect(&self, other: &BoundingBox) -> BoundingBox {
        let start = Point3::new(
            max(self.start.x, other.start.x),
            max(self.start.y, other.start.y),
            max(self.start.z, other.start.z),
        );
        let end = Point3::new(
            max(min(self.end.x, other.end.x), start.x),
            max(min(self.end.y, other.end.y), start.y),
            max(min(self.end.z, other.end.z), start.z),
        );
        BoundingBox::new(start, end)
    }

    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox::containing_points([self.start, self.end, other.start, other.end].iter())
    }

    pub fn get_overlap(&self, other: &BoundingBox) -> OverlapResult {
        // eprintln!("self = {:?}", self);
        // eprintln!("other = {:?}", other);
        let union = self.union(other);
        // eprintln!("union = {:?}", union);
        let intersection = self.intersect(other);
        if self == other {
            OverlapResult::Identical
        } else if union == *other {
            OverlapResult::FullyContainedByArg
        } else if union == *self {
            OverlapResult::FullyContainsArg
        } else if !intersection.is_empty() {
            // Calculate the best mid-points for each.
            OverlapResult::PartialOverlap {
                self_mid_point: self.get_best_mid_point(&intersection),
                arg_mid_point: other.get_best_mid_point(&intersection),
            }
        } else {
            OverlapResult::NoOverlap
        }
    }

    pub fn volume(&self) -> u64 {
        let delta = self.end - self.start;
        delta.dx as u64 * delta.dy as u64 * delta.dz as u64
    }

    pub fn get_best_mid_point(&self, mid_point_options: &BoundingBox) -> Point3 {
        let points_by_score = mid_point_options
            .corners()
            .into_iter()
            .map(|p| (self.score_mid_point(&p), p))
            .into_group_map();
        let max_score = points_by_score.keys().max().unwrap();
        points_by_score.get(max_score).unwrap()[0]
    }

    fn score_mid_point(&self, point: &Point3) -> u32 {
        (min(point.x - self.start.x, self.end.x - point.x)
            + min(point.y - self.start.y, self.end.y - point.y)
            + min(point.z - self.start.z, self.end.z - point.z)) as u32
    }
}

#[cfg(test)]
mod test {
    use crate::util::point3::{BoundingBox, OverlapResult, Point3};

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
