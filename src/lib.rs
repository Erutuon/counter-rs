//! Counter based on the Python implementation of same:
//! <https://docs.python.org/3.5/library/collections.html#collections.Counter>
//!
//! Counts recurring elements from an iterable.

use std::collections::HashMap;
use std::hash::Hash;

use std::ops::{Add, Sub, BitAnd, BitOr};

#[derive(Clone)]
pub struct Counter<'a, T: 'a> {
    /// HashMap backing this Counter
    ///
    /// Public to expose the HashMap API for direct manipulation.
    /// That said, this may change in the future to some other mapping type / trait.
    pub map: HashMap<&'a T, usize>,
}

impl<'a, T> Counter<'a, T>
    where T: 'a + Hash + Eq
{
    /// Create a new, empty `Counter`
    pub fn new() -> Counter<'a, T> {
        Counter { map: HashMap::new() }
    }

    /// Create a new `Counter` initialized with the given iterable
    pub fn init<I>(iterable: I) -> Counter<'a, T>
        where I: IntoIterator<Item = &'a T>
    {
        let mut counter = Counter::new();
        counter.update(iterable);
        counter
    }

    /// Add the counts of the elements from the given iterable to this counter
    pub fn update<I>(&mut self, iterable: I)
        where I: IntoIterator<Item = &'a T>
    {
        for item in iterable.into_iter() {
            let entry = self.map.entry(item).or_insert(0);
            *entry += 1;
        }
    }

    /// Remove the counts of the elements from the given iterable to this counter
    ///
    /// Non-positive counts are automatically removed
    pub fn subtract<I>(&mut self, iterable: I)
        where I: IntoIterator<Item = &'a T>
    {
        for item in iterable.into_iter() {
            let mut remove = false;
            if let Some(entry) = self.map.get_mut(item) {
                if *entry > 0 {
                    *entry -= 1;
                }
                remove = *entry == 0;
            }
            if remove {
                self.map.remove(item);
            }
        }
    }

    /// Create an iterator over `(frequency, elem)` pairs, sorted most to least common.
    ///
    /// FIXME: This is pretty inefficient: it copies everything into a vector, sorts
    /// the vector, and returns an iterator over the vector. It would be much better
    /// to create some kind of MostCommon struct which implements `Iterator` which
    /// does all the necessary work on demand. PRs appreciated here!
    pub fn most_common(&self) -> ::std::vec::IntoIter<(&&T, &usize)> {
        let mut items = self.map.iter().collect::<Vec<_>>();
        items.sort_by(|&(_, a), &(_, b)| b.cmp(a));
        items.into_iter()
    }
}

impl<'a, T> Add for Counter<'a, T>
    where T: Clone + Hash + Eq
{
    type Output = Counter<'a, T>;

    /// Add two counters together.
    ///
    /// `out = c + d;` -> `out[x] == c[x] + d[x]`
    fn add(self, rhs: Counter<'a, T>) -> Counter<'a, T> {
        let mut counter = self.clone();
        for (key, value) in rhs.map.iter() {
            let entry = counter.map.entry(key).or_insert(0);
            *entry += *value;
        }
        counter
    }
}

impl<'a, T> Sub for Counter<'a, T>
    where T: Clone + Hash + Eq
{
    type Output = Counter<'a, T>;

    /// Subtract (keeping only positive values).
    ///
    /// `out = c - d;` -> `out[x] == c[x] - d[x]`
    fn sub(self, rhs: Counter<'a, T>) -> Counter<'a, T> {
        let mut counter = self.clone();
        for (key, value) in rhs.map.iter() {
            let mut remove = false;
            if let Some(entry) = counter.map.get_mut(key) {
                if *entry >= *value {
                    *entry -= *value;
                } else {
                    remove = true;
                }
                if *entry == 0 {
                    remove = true;
                }
            }
            if remove {
                counter.map.remove(key);
            }
        }
        counter
    }
}

impl<'a, T> BitAnd for Counter<'a, T>
    where T: Clone + Hash + Eq
{
    type Output = Counter<'a, T>;

    /// Intersection
    ///
    /// `out = c & d;` -> `out[x] == min(c[x], d[x])`
    fn bitand(self, rhs: Counter<'a, T>) -> Counter<'a, T> {
        use std::cmp::min;
        use std::collections::HashSet;

        let self_keys = self.map.keys().collect::<HashSet<_>>();
        let other_keys = rhs.map.keys().collect::<HashSet<_>>();
        let both_keys = self_keys.intersection(&other_keys);

        let mut counter = Counter::new();
        for key in both_keys {
            counter.map.insert(**key,
                               min(*self.map.get(*key).unwrap(), *rhs.map.get(*key).unwrap()));
        }

        counter
    }
}

impl<'a, T> BitOr for Counter<'a, T>
    where T: Clone + Hash + Eq
{
    type Output = Counter<'a, T>;

    /// Union
    ///
    /// `out = c | d;` -> `out[x] == max(c[x], d[x])`
    fn bitor(self, rhs: Counter<'a, T>) -> Counter<'a, T> {
        use std::cmp::max;

        let mut counter = self.clone();
        for (key, value) in rhs.map.iter() {
            let entry = counter.map.entry(key).or_insert(0);
            *entry = max(*entry, *value);
        }
        counter
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
