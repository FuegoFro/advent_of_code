use crate::util::point3::{BoundingBox, OverlapResult, Point3};
use itertools::Itertools;
use recap::Recap;
use serde::de::Unexpected;
use serde::{de, Deserialize, Deserializer};
use std::fmt::{Debug, Formatter};

#[derive(Deserialize, Recap, Debug)]
// eg "on x=-20..26,y=-36..17,z=-47..7"
#[recap(
    regex = r"(?P<on>on|off) x=(?P<x_start>-?\d+)..(?P<x_end>-?\d+),y=(?P<y_start>-?\d+)..(?P<y_end>-?\d+),z=(?P<z_start>-?\d+)..(?P<z_end>-?\d+)"
)]
struct Instruction {
    #[serde(deserialize_with = "bool_from_string")]
    on: bool,
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    z_start: i32,
    z_end: i32,
}

#[derive(Clone, Debug)]
struct Modification {
    on: bool,
    bound: BoundingBox,
}

impl Modification {
    const INITIAL_RANGE: BoundingBox =
        BoundingBox::new(Point3::new(-50, -50, -50), Point3::new(50, 50, 50));

    fn new(on: bool, bound: BoundingBox) -> Self {
        Modification { on, bound }
    }

    fn from_instruction(instruction: Instruction) -> Self {
        Modification {
            on: instruction.on,
            bound: BoundingBox::new(
                Point3::new(
                    instruction.x_start,
                    instruction.y_start,
                    instruction.z_start,
                ),
                Point3::new(
                    instruction.x_end + 1,
                    instruction.y_end + 1,
                    instruction.z_end + 1,
                ),
            ),
        }
    }

    fn is_in_initial_range(&self) -> bool {
        matches!(
            Modification::INITIAL_RANGE.get_overlap(&self.bound),
            OverlapResult::FullyContainsArg
        )
    }

    fn split_at(&self, point: &Point3) -> [Modification; 8] {
        let clamped_point = point.clamp(&self.bound);
        self.bound
            .octants(&clamped_point)
            .map(|bound| Modification::new(self.on, bound))
    }
}

/// Deserialize bool from String with custom value mapping
fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "on" => Ok(true),
        "off" => Ok(false),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"on or off",
        )),
    }
}

/// Start has the lowest values (all of its coords are lowest), end has the highest
/// Values is ordered with first value start and last value at end, increasing x then y then z
struct OctTreeNode {
    bound: BoundingBox,
    mid: Point3,
    values: [OctTreeValue; 8],
}

impl OctTreeNode {
    fn new(bound: BoundingBox, mid: Point3, on: bool) -> Self {
        OctTreeNode {
            bound,
            mid,
            values: [on; 8].map(OctTreeValue::Uniform),
        }
    }

    fn sections_mut(&mut self) -> [(BoundingBox, &mut OctTreeValue); 8] {
        self.bound.octants(&self.mid).zip(self.values.each_mut())
    }

    fn sections(&self) -> [(BoundingBox, &OctTreeValue); 8] {
        self.bound.octants(&self.mid).zip(self.values.each_ref())
    }

    /// For each node section
    ///     Re-calculate mod w/ just overlapping part
    ///     If mod is empty (doesn't overlap at all), skip
    ///     If mod fully overlaps, set to on/off value
    ///     If mod partially overlaps, insert into value
    fn insert(&mut self, modification: Modification, depth: usize) {
        // println!("({}) ==== Inserting into node", depth);
        // println!("({}) modification = {:?}", depth, modification);
        // println!("({}) self.bound = {:?}", depth, self.bound);
        // println!("({}) self.mid = {:?}", depth, self.mid);
        let sections_and_mods = modification.split_at(&self.mid).zip(self.sections_mut());
        // println!("({}) sections_and_mods = [", depth);
        // for section in sections_and_mods.iter() {
        //     println!("  {:?},", section);
        // }
        // println!("];",);
        for (_, (modification, (bound, value))) in sections_and_mods.into_iter().enumerate() {
            if modification.bound.is_empty() {
                continue;
            }
            // println!("({}) ---- Section info - {}", depth, idx);
            // println!("({}) modification = {:?}", depth, modification);
            // println!("({}) bound = {:?}", depth, bound);
            // println!("({}) value = {:?}", depth, value);

            match modification.bound.get_overlap(&bound) {
                OverlapResult::Identical => {
                    *value = OctTreeValue::Uniform(modification.on);
                }
                OverlapResult::FullyContainsArg => {
                    unreachable!("Modification should be divided into small enough bounds by now");
                }
                OverlapResult::FullyContainedByArg => {
                    let mid_point = bound.get_best_mid_point(&modification.bound);
                    value.insert_partial_overlap(modification, bound, mid_point, depth);
                }
                OverlapResult::PartialOverlap { arg_mid_point, .. } => {
                    value.insert_partial_overlap(modification, bound, arg_mid_point, depth);
                }
                OverlapResult::NoOverlap => {
                    panic!("Shouldn't be called with a no-overlap modification");
                }
            }
        }
    }

    fn total_volume(&self) -> u64 {
        self.sections()
            .into_iter()
            .map(|(bound, value)| match value {
                OctTreeValue::Uniform(on) => {
                    if *on {
                        bound.volume()
                    } else {
                        0
                    }
                }
                OctTreeValue::Divided(child) => child.total_volume(),
            })
            .sum()
    }

    #[allow(dead_code)]
    fn print(&self, indent: usize) {
        for (bound, value) in self.sections() {
            match value {
                OctTreeValue::Uniform(on) => {
                    if bound.volume() > 0 {
                        println!("{:width$} {} {:?}", "-", on, bound, width = indent * 4);
                    }
                }
                OctTreeValue::Divided(child) => child.print(indent + 1),
            }
        }
    }
}

enum OctTreeValue {
    Uniform(bool),
    Divided(Box<OctTreeNode>),
}

impl OctTreeValue {
    /// If was uniform
    ///     If mod matches existing state, skip
    ///     Else split w/ midpoint at point that's inside the section.
    ///         If no point, construct one from the line/plane, set as child
    ///         Fall through to below
    /// If child, recurse, then simplify if trivial
    fn insert_partial_overlap(
        &mut self,
        modification: Modification,
        bound: BoundingBox,
        mid_point: Point3,
        depth: usize,
    ) {
        // if depth > 1 {
        //     println!("TOO DEEP, STOPPING");
        //     return;
        // }
        // println!("({}) ---- insert_partial_overlap", depth);
        // println!("({}) modification = {:?}", depth, modification);
        // println!("({}) bound = {:?}", depth, bound);
        // println!("({}) mid_point = {:?}", depth, mid_point);
        let child = match self {
            OctTreeValue::Uniform(was_on) => {
                if *was_on == modification.on {
                    // We already match, move on
                    return;
                }

                let new_child = OctTreeNode::new(bound, mid_point, *was_on);
                *self = OctTreeValue::Divided(Box::new(new_child));
                // TODO - Is there a way to avoid this?
                match self {
                    OctTreeValue::Uniform(_) => unreachable!(),
                    OctTreeValue::Divided(child) => child,
                }
            }

            OctTreeValue::Divided(child) => child,
        };

        child.insert(modification, depth + 1);

        // TODO - Simplify
    }
}

impl Debug for OctTreeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OctTreeValue::Uniform(on) => f.debug_tuple("OctTreeValue::Uniform").field(on).finish(),
            OctTreeValue::Divided(_) => f.debug_tuple("OctTreeValue::Divided").field(&"_").finish(),
        }
    }
}

/// For each mod
///     If the mod isn't fully contained, expand the root node
///         Have existing node be one quadrant
///     Insert into root node
fn run_sequence(modifications: Vec<Modification>, only_initial: bool) -> OctTreeNode {
    let mut root = OctTreeNode::new(BoundingBox::EMPTY, BoundingBox::EMPTY.start, false);
    // for modification in modifications.into_iter().take(2) {
    for modification in modifications {
        if only_initial && !modification.is_in_initial_range() {
            continue;
        }
        // println!("Inserting mod {:?}", modification);
        if root.bound.is_empty() {
            let mid = modification.bound.start;
            root = OctTreeNode::new(modification.bound, mid, modification.on);
            continue;
        }

        match modification.bound.get_overlap(&root.bound) {
            OverlapResult::FullyContainedByArg => {
                // Just insert
                root.insert(modification, 0);
            }
            OverlapResult::Identical | OverlapResult::FullyContainsArg => {
                // Replace w/ fresh root node
                let new_bounds = if modification.on {
                    modification.bound
                } else {
                    BoundingBox::EMPTY
                };
                let mid = new_bounds.start;
                root = OctTreeNode::new(new_bounds, mid, modification.on);
            }
            OverlapResult::PartialOverlap { .. } | OverlapResult::NoOverlap => {
                // Expand, using best point of existing node bound
                let outer_bound = modification.bound.union(&root.bound);
                let mid_point = outer_bound.get_best_mid_point(&root.bound);
                let mut new_root = OctTreeNode::new(outer_bound, mid_point, false);
                let mut matching_sections = new_root
                    .sections_mut()
                    .into_iter()
                    .filter(|(bound, _)| *bound == root.bound)
                    .collect_vec();
                assert_eq!(matching_sections.len(), 1);
                let (_, value) = matching_sections.pop().unwrap();
                *value = OctTreeValue::Divided(Box::new(root));
                root = new_root;
                root.insert(modification, 0);
            }
        }
        // root.print(0);
    }
    root
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let modifications = input
        .split("\n")
        .map(|l| l.parse::<Instruction>().unwrap())
        .map(Modification::from_instruction)
        .collect_vec();

    // let mod_bound = BoundingBox {
    //     start: Point3 { x: 2, y: 0, z: 2 },
    //     end: Point3 { x: 4, y: 2, z: 4 },
    // };
    // let bound = BoundingBox {
    //     start: Point3 { x: 2, y: 0, z: 0 },
    //     end: Point3 { x: 6, y: 2, z: 4 },
    // };
    // dbg!(mod_bound.get_overlap(&bound));
    // return;

    // Example answer: 590784
    // Actual answer:  546724
    println!(
        "Part 1: {}",
        run_sequence(modifications.clone(), true).total_volume() // run_sequence(modifications.clone(), false).total_volume()
    );
    println!(
        "Part 2: {}",
        run_sequence(modifications.clone(), false).total_volume()
    );
}

/*
on x=-12..41,y=-1..48,z=-27..19
on x=-40..7,y=-47..2,z=-24..22

on x=2..6,y=2..6,z=0..4
on x=0..4,y=0..4,z=2..6



x -40 -12  7 41
y -47  -1  2 48
z -27 -24 19 22


BoundingBox {
    start: Point3 {
        x: -12,
        y: -1,
        z: -24,
    },
    end: Point3 {
        x: 7,
        y: 2,
        z: 19,
    },
}

*/
