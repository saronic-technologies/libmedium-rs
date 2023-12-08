use super::*;

use std::{collections::btree_map::Iter as BTreeIter, iter::FusedIterator};

type InnerIter<'a> = BTreeIter<'a, u16, Hwmon>;

/// An iterator over all parsed hwmons, their names and indices.
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    inner: InnerIter<'a>,
}

impl<'a> Iter<'a> {
    pub(super) fn new(inner: InnerIter<'a>) -> Self {
        Self { inner }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Hwmon;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, hwmon)) = self.inner.next() {
            return Some(hwmon);
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a> FusedIterator for Iter<'a> {}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a> IntoIterator for &'a Hwmons {
    type Item = &'a Hwmon;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over all parsed hwmons with a given name and their indices.
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct NamedIter<'a, N> {
    inner: Iter<'a>,
    name: N,
}

impl<'a, N: AsRef<str>> NamedIter<'a, N> {
    pub(super) fn new(inner: Iter<'a>, name: N) -> Self {
        Self { inner, name }
    }
}

impl<'a, N: AsRef<str>> Iterator for NamedIter<'a, N> {
    type Item = &'a Hwmon;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(hwmon) => {
                if hwmon.name() == self.name.as_ref() {
                    Some(hwmon)
                } else {
                    self.next()
                }
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.inner.size_hint().1)
    }
}

impl<'a, N: AsRef<str>> FusedIterator for NamedIter<'a, N> {}
