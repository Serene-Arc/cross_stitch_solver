use itertools::{Itertools, Permutations};
use std::hash::Hash;
use std::vec::IntoIter;

pub struct PrefixedPermutations<T>
where
    T: Clone + Hash + PartialEq + Eq,
{
    prefix: Option<T>,
    inner: Permutations<IntoIter<T>>,
    pub free_elements: i64,
}

impl<T: Clone + Hash + PartialEq + Eq> PrefixedPermutations<T> {
    pub fn new(prefix: Option<T>, inner: Vec<T>) -> Self {
        Self {
            free_elements: { (&inner).len() as i64 },
            prefix,
            inner: inner.clone().into_iter().permutations((&inner).len()),
        }
    }
}

impl<T: Clone + Hash + PartialEq + Eq> Iterator for PrefixedPermutations<T> {
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

                Some(result)
            }
            None => None,
        }
    }
}
#[cfg(test)]
mod test {
    use crate::csv_reader::generate_permutations;
    use crate::stitch::{make_full_stitch, HalfStitch, Location};

    #[test]
    fn test_permutation_generation_first_element_consistent() {
        let first = HalfStitch::new(Location::new(1, 1), true);
        let test = vec![
            HalfStitch::new(Location::new(2, 2), true),
            HalfStitch::new(Location::new(3, 1), true),
            HalfStitch::new(Location::new(4, 1), true),
        ];
        let perms = generate_permutations(Some(first), test);
        for p in perms {
            assert_eq!(p[0], first);
        }
    }

    #[test]
    fn test_permutation_generation_first_element_not_consistent() {
        let test = vec![
            HalfStitch::new(Location::new(1, 1), true),
            HalfStitch::new(Location::new(2, 2), true),
            HalfStitch::new(Location::new(3, 1), true),
        ];
        let perms = generate_permutations(None, test.clone());

        let mut is_different_first_elem_found = false;
        for perm in perms {
            if &perm[0] != &test[0] {
                is_different_first_elem_found = true;
                break;
            }
        }
        assert!(
            is_different_first_elem_found,
            "All permutations start with the same element"
        );
    }

    #[test]
    fn test_permutation_generation_specific_permutation_base() {
        let test = vec![
            HalfStitch::new(Location::new(1, 1), true),
            HalfStitch::new(Location::new(2, 2), true),
            HalfStitch::new(Location::new(3, 1), true),
        ];
        let perms = generate_permutations(None, test.clone());

        let mut found = false;
        for perm in perms {
            if perm == test {
                found = true;
                break;
            }
        }
        assert!(found, "Expected permutation was not found");
    }

    #[test]
    fn test_permutation_generation_specific_permutation_kick() {
        let test = [
            make_full_stitch(1, 1),
            make_full_stitch(2, 1),
            make_full_stitch(3, 2),
        ]
        .concat();
        let perms = generate_permutations(None, test.clone());

        let expected = vec![
            HalfStitch::new(Location::new(1, 1), true),
            HalfStitch::new(Location::new(2, 1), true),
            HalfStitch::new(Location::new(3, 1), false),
            HalfStitch::new(Location::new(3, 2), true),
            HalfStitch::new(Location::new(4, 2), false),
            HalfStitch::new(Location::new(2, 1), false),
        ];
        let mut found = false;
        for perm in perms {
            if perm == expected {
                found = true;
                break;
            }
        }
        assert!(found, "Expected permutation was not found");
    }
}
