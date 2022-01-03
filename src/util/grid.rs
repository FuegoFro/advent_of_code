#![allow(dead_code)]

use std::fmt::{Debug, Formatter, Write};
use std::ops::{Index, IndexMut};

use itertools::Itertools;

use crate::util::point2::{Delta, PointU};

#[derive(Clone)]
pub struct Grid<T> {
    storage: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> PartialEq<Self> for Grid<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.storage.eq(&other.storage)
    }

    fn ne(&self, other: &Self) -> bool {
        self.storage.ne(&other.storage)
    }
}

impl<T> Eq for Grid<T> where T: Eq {}

pub enum Neighbors {
    Four,
    Eight,
}

impl<T> Grid<T>
where
    T: Default,
{
    pub fn empty(width: usize, height: usize) -> Self {
        let storage = (0..height)
            .map(|_| (0..width).map(|_| Default::default()).collect_vec())
            .collect_vec();
        Self::from_storage(storage)
    }
}

impl<T> Grid<T> {
    pub fn from_storage(storage: Vec<Vec<T>>) -> Self {
        Grid {
            // Assumes equal-length rows
            height: storage.len(),
            width: storage.get(0).map_or(0, |row| row.len()),
            storage,
        }
    }

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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
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
        self.neighbors(point, neighbors)
            .map(|neighbor_point| (neighbor_point, &self[neighbor_point]))
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

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let strs = self
            .storage
            .iter()
            .map(|row| row.iter().map(|v| format!("{:?}", v)).collect_vec())
            .collect_vec();

        let longest = strs
            .iter()
            .flat_map(|row| row.iter())
            .map(|v| v.len())
            .max()
            .unwrap_or(0);

        let mut final_string = String::new();
        for row in strs {
            final_string.write_str("  ")?;
            for (i, v) in row.into_iter().enumerate() {
                if i != 0 {
                    final_string.push(' ')
                }
                final_string.write_fmt(format_args!("{:>width$}", v, width = longest))?;
            }
            final_string.push('\n');
        }
        f.debug_struct("Grid")
            .field("storage", &format_args!("\n{}", final_string))
            .finish()
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
