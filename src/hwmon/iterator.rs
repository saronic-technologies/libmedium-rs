use super::*;

use std::{collections::btree_map::Iter as BTreeIter, iter::FusedIterator};

type InnerIter<'a> = BTreeIter<'a, u16, Hwmon>;

/// An iterator over all parsed hwmons.
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    hwmons: InnerIter<'a>,
}

impl<'a> Iter<'a> {
    pub(super) fn new(hwmons: InnerIter<'a>) -> Self {
        Self { hwmons }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (u16, &'a str, &'a Hwmon);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, hwmon)) = self.hwmons.next() {
            return Some((*index, hwmon.name(), hwmon));
        }
        None
    }
}

impl<'a> FusedIterator for Iter<'a> {}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.hwmons.len()
    }
}

impl<'a> IntoIterator for &'a Hwmons {
    type Item = (u16, &'a str, &'a Hwmon);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
