use num_traits::Num;
use std::collections::HashMap;
use std::hash::Hash;

pub trait IteratorHelpers: Iterator {
    fn into_count_map<I>(self) -> HashMap<Self::Item, usize>
    where
        Self: Iterator<Item = I> + Sized,
        I: Hash + Eq,
    {
        let mut count_map = HashMap::new();
        for item in self {
            *count_map.entry(item).or_default() += 1;
        }
        count_map
    }

    fn into_sum_map<K, V>(self) -> HashMap<K, V>
    where
        Self: Iterator<Item = (K, V)> + Sized,
        K: Hash + Eq,
        V: Num + Default + Copy,
    {
        let mut sum_map = HashMap::new();
        for (key, count) in self {
            let entry = sum_map.entry(key).or_default();
            *entry = *entry + count;
        }
        sum_map
    }
}

impl<T: ?Sized> IteratorHelpers for T where T: Iterator {}
