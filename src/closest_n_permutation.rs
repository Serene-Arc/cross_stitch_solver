use crate::stitch::{HalfStitch, Location};
use itertools::{Itertools, MultiProduct};
use std::collections::HashMap;
use std::iter::repeat;
use std::vec::IntoIter;

pub struct ClosestNElementsIterator {
    // Cache of closest stitches to the end location of the given stitch, not including invalid stitches
    cache: HashMap<HalfStitch, Vec<HalfStitch>>,
    closest_n_iterator: MultiProduct<IntoIter<usize>>,
    count: usize,
    first_location: Option<HalfStitch>,
    n_value: usize,
    values: Vec<HalfStitch>,
}

impl ClosestNElementsIterator {
    pub fn new(
        first_loc: Option<HalfStitch>,
        values: Vec<HalfStitch>,
        closest_n_value: usize,
    ) -> Self {
        let iterator = repeat((0..closest_n_value).collect::<Vec<usize>>())
            .take(values.len() - 1)
            .multi_cartesian_product();
        Self {
            first_location: first_loc,
            values,
            count: 0,
            n_value: closest_n_value,
            closest_n_iterator: iterator,
            cache: HashMap::new(),
        }
    }
    fn find_n_closest_stitches(&mut self, stitch: &HalfStitch) -> Vec<HalfStitch> {
        if let Some(closest_points) = self.cache.get(stitch) {
            return closest_points.clone();
        }

        let distances = self
            .values
            .iter()
            .map(|v| {
                (
                    (((v.start.x - stitch.get_end_location().x).pow(2)
                        + (v.start.y - stitch.get_end_location().y).pow(2))
                        as f64)
                        .sqrt(),
                    v.clone(),
                )
            })
            .filter(|l| l.1.start != stitch.get_end_location())
            .sorted_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .collect::<Vec<(f64, HalfStitch)>>();

        let closest_points: Vec<HalfStitch> = distances.into_iter().map(|v| v.1).clone().collect();

        self.cache.insert(*stitch, closest_points.clone());

        closest_points
    }

    fn get_nth_unused_closest_stitch(
        &mut self,
        location: &HalfStitch,
        n: usize,
        visited_stitches: &Vec<HalfStitch>,
    ) -> HalfStitch {
        let mut closest_locations = self
            .find_n_closest_stitches(location)
            .into_iter()
            .filter(|l| !visited_stitches.contains(l));
        let out_location = closest_locations.nth(n);
        match out_location {
            None => closest_locations
                .last()
                .expect("Could not get last element of iterator")
                .clone(),
            Some(location) => location.clone(),
        }
    }
}

impl Iterator for ClosestNElementsIterator {
    type Item = Vec<HalfStitch>;

    fn next(&mut self) -> Option<Self::Item> {
        let n_sequence = self.closest_n_iterator.next();
        match n_sequence {
            None => None,
            Some(sequence) => {
                let mut out = Vec::with_capacity(self.values.len() + 1);
                match self.first_location {
                    None => {}
                    Some(first_loc) => {
                        out[0] = first_loc;
                    }
                }
                for index in sequence {
                    out.push(self.get_nth_unused_closest_stitch(
                        out.last().expect("Found an empty vector"),
                        index,
                        &out,
                    ))
                }
                self.count += 1;
                Some(out)
            }
        }
    }
}

impl ExactSizeIterator for ClosestNElementsIterator {
    fn len(&self) -> usize {
        (self.n_value * (self.values.len() - 1)) - self.count
    }
}
