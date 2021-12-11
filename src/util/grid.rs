#![allow(dead_code)]

use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

use itertools::Itertools;

use crate::util::point2::{Delta, PointU};

#[derive(Clone)]
pub struct Grid<T> {
    storage: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

pub enum Neighbors {
    Four,
    Eight,
}

impl<T> Grid<T> {
    pub fn from_str(
        raw: impl AsRef<str>,
        row_delimiter: &str,
        col_delimiter: Option<&str>,
        col_transform: impl Fn(&str) -> T,
    ) -> Self {
        let storage = raw
            .as_ref()
            .split(row_delimiter)
            .map(|row| {
                // Need to duplicate map and collect because iterator types differ and we
                // need to do some shenanigans to make char's into &str's
                match col_delimiter {
                    Some(col_delimiter) => {
                        row.split(col_delimiter).map(&col_transform).collect_vec()
                    }
                    None => {
                        let char_strings = row.chars().map(|c| c.to_string());
                        char_strings
                            .map(|s| col_transform(s.as_str()))
                            .collect_vec()
                    }
                }
            })
            .collect_vec();
        Grid {
            // Assumes equal-length rows
            height: storage.len(),
            width: storage.get(0).map_or(0, |row| row.len()),
            storage,
        }
    }

    pub fn points(&self) -> impl Iterator<Item = PointU> {
        // Assumes even-length rows
        let height = self.height;
        let width = self.width;
        (0..height).flat_map(move |y| (0..width).map(move |x| PointU::new(x, y)))
    }

    pub fn neighbors_with_values(
        &self,
        point: PointU,
        neighbors: Neighbors,
    ) -> impl Iterator<Item = (PointU, &T)> {
        match neighbors {
            Neighbors::Four => &Delta::NEIGHBORS4[..],
            Neighbors::Eight => &Delta::NEIGHBORS8[..],
        }
        .iter()
        .map(move |delta| {
            let neighbor_point = point + delta;
            (neighbor_point, &self[neighbor_point])
        })
    }

    pub fn neighbors(&self, point: PointU, neighbors: Neighbors) -> impl Iterator<Item = PointU> {
        match neighbors {
            Neighbors::Four => &Delta::NEIGHBORS4[..],
            Neighbors::Eight => &Delta::NEIGHBORS8[..],
        }
        .iter()
        .filter(move |delta| {
            (point.x > 0 || delta.dx >= 0)
                && (point.x < self.width - 1 || delta.dx <= 0)
                && (point.y > 0 || delta.dy >= 0)
                && (point.y < self.height - 1 || delta.dy <= 0)
        })
        .map(move |delta| point + delta)
        .collect_vec()
        .into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.storage.iter_mut().flat_map(|row| row.iter_mut())
    }

    pub fn iter_with_points(&self) -> impl Iterator<Item = (PointU, &T)> {
        self.storage.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, v)| (PointU::new(x, y), v))
        })
    }
}

impl<T> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
        /*
        fn print_grid(grid: &Vec<Vec<u32>>) {
            println!(
                "{}",
                grid.iter()
                    .map(|r| r
                        .iter()
                        .map(|v| if *v > 9 {
                            "x".to_string()
                        } else {
                            v.to_string()
                        })
                        .join(""))
                    .join("\n")
            );
        }
        */
    }
}

impl<T> Index<PointU> for Grid<T> {
    type Output = T;

    fn index(&self, index: PointU) -> &Self::Output {
        &self.storage[index.y][index.x]
    }
}

impl<T> IndexMut<PointU> for Grid<T> {
    fn index_mut(&mut self, index: PointU) -> &mut Self::Output {
        &mut self.storage[index.y][index.x]
    }
}
