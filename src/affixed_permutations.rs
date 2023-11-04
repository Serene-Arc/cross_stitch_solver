use itertools::{Itertools, Permutations};
use std::hash::Hash;
use std::vec::IntoIter;

pub struct AffixedPermutations<T>
where
    T: Clone + Hash + PartialEq + Eq,
{
    prefix: Option<T>,
    suffix: Option<T>,
    inner: Permutations<IntoIter<T>>,
    pub free_elements: i64,
}

impl<T: Clone + Hash + PartialEq + Eq> AffixedPermutations<T> {
    pub fn new(prefix: Option<T>, suffix: Option<T>, inner: Vec<T>) -> Self {
        Self {
            free_elements: { (&inner).len() as i64 },
            prefix,
            suffix,
            inner: inner.clone().into_iter().permutations((&inner).len()),
        }
    }
}

impl<T: Clone + Hash + PartialEq + Eq> Iterator for AffixedPermutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(mut middle) => {
                let mut result: Vec<T> = Vec::with_capacity(middle.len() + 2);
                match &self.prefix {
                    None => {}
                    Some(i) => result.push(i.clone()),
                }

                result.append(&mut middle);

                match &self.suffix {
                    None => {}
                    Some(i) => result.push(i.clone()),
                }
                Some(result)
            }
            None => None,
        }
    }
}
