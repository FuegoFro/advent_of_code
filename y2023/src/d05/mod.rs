use itertools::Itertools;
use std::cmp::min;
use std::ops::Range;
use util::p_u64;

#[derive(Debug)]
struct RangeInfo {
    src_range: Range<u64>,
    dst_start: u64,
}

impl RangeInfo {
    fn from_line(line: &str) -> Self {
        let (dst_start, src_start, len) =
            line.split_whitespace().map(p_u64).collect_tuple().unwrap();
        RangeInfo {
            src_range: src_start..(src_start + len),
            dst_start,
        }
    }
}

struct SectionInfo {
    ranges: Vec<RangeInfo>,
}

impl SectionInfo {
    // fn translate(&self, val: u64) -> u64 {
    //     for range in self.ranges.iter() {
    //         if range.src_range.contains(&val) {
    //             return range.dst_start + (val - range.src_range.start);
    //         }
    //     }
    //     val
    // }

    fn split_range(&self, mut range: Range<u64>) -> Vec<Range<u64>> {
        // let initial = range.clone();
        let mut results = vec![];
        let mut translations = self.ranges.iter();
        let mut translation = translations.next();
        // split off beginning of range
        while !range.is_empty() {
            // while the map ends before our start, advance the map
            while let Some(t) = translation {
                // Would love to have it be `&& t.src_range.end <= range.start` in the while
                if t.src_range.end > range.start {
                    break;
                }
                // println!("Advancing {:?} ({:?})", t, range);
                translation = translations.next()
            }

            // if we're in a map, go until the earlier of the end of our range or the end of the map
            let mut did_translate = false;
            // println!("Checking {:?} with {:?}", range, translation);
            if let Some(t) = translation {
                if t.src_range.contains(&range.start) {
                    // println!("Translating {:?} with {:?}", range, t);
                    // do translation
                    let offset = range.start - t.src_range.start;
                    let translated_start = t.dst_start + offset;

                    let end = min(range.end, t.src_range.end);
                    let len = end - range.start;
                    let translated_end = translated_start + len;

                    results.push(translated_start..translated_end);
                    range.start = end;
                    did_translate = true;
                }
            }
            // if we're not in a map, go until the earlier of the end of our range or the beginning
            // of the next map (if there is a next map)
            if !did_translate {
                let end = min(
                    range.end,
                    translation.map(|t| t.src_range.start).unwrap_or(range.end),
                );
                results.push(range.start..end);
                range.start = end;
            }
        }
        // println!("{:?} -> {:?}", initial, results);
        results
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let sections = input.split("\n\n").collect_vec();
    let (seeds_raw, rest) = sections.split_first().unwrap();
    let seeds_pt1 = seeds_raw
        .split_whitespace()
        .skip(1)
        .map(p_u64)
        .map(|n| n..(n + 1))
        .collect_vec();
    let seeds_pt2 = seeds_raw
        .split_whitespace()
        .skip(1)
        .map(p_u64)
        .chunks(2)
        .into_iter()
        .map(|c| {
            let (start, len) = c.collect_tuple().unwrap();
            start..(start + len)
        })
        .collect_vec();
    let maps = rest
        .iter()
        .map(|section| {
            let ranges = section
                .lines()
                .skip(1)
                .map(RangeInfo::from_line)
                .sorted_by_key(|r| (r.src_range.start, r.src_range.end))
                .collect_vec();
            SectionInfo { ranges }
        })
        .collect_vec();

    println!("Part 1: {}", get_lowest(&maps, seeds_pt1));
    println!("Part 2: {}", get_lowest(&maps, seeds_pt2));
}

fn get_lowest(sections: &[SectionInfo], seeds: Vec<Range<u64>>) -> u64 {
    sections
        .iter()
        .fold(seeds, |ranges, section| {
            ranges
                .into_iter()
                .flat_map(|range| section.split_range(range))
                .collect_vec()
        })
        .iter()
        .map(|r| r.start)
        .min()
        .unwrap()
    // let closest = seeds
    //     .iter()
    //     .map(|s| {
    //         sections.iter()
    //             .fold(*s, |current, section_info| section_info.translate(current))
    //     })
    //     .min()
    //     .unwrap();
    // closest
}
