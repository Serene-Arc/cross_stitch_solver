use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, Serialize)]
pub struct Location {
    x: i64,
    y: i64,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct HalfStitch {
    // the start is the base of the stitch, wherever that is.
    start: Location,
    facing_right: bool,
}

impl Location {
    fn distance_to_location(&self, other: &Location) -> f64 {
        let x_dist: f64 = (self.x as i64 - other.x as i64) as f64;
        let y_dist: f64 = (self.y as i64 - other.y as i64) as f64;
        (x_dist * x_dist + y_dist * y_dist).sqrt()
    }
    pub fn new(x: i64, y: i64) -> Location {
        Location { x, y }
    }
}

impl HalfStitch {
    pub fn get_end_location(&self) -> Location {
        if self.facing_right {
            Location::new(self.start.x + 1, self.start.y + 1)
        } else {
            Location::new(self.start.x - 1, self.start.y + 1)
        }
    }
    pub(crate) fn new(start: Location, facing_right: bool) -> Self {
        HalfStitch {
            start,
            facing_right,
        }
    }
}

impl Serialize for HalfStitch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("HalfStitch", 3)?;
        state.serialize_field("start_x", &self.start.x)?;
        state.serialize_field("start_y", &self.start.y)?;
        state.serialize_field("facing_right", &self.facing_right)?;
        state.end()
    }
}

pub fn get_cost(stitches: &Vec<HalfStitch>, end_location: &Option<Location>) -> f64 {
    let mut cost: f64 = 0.0;
    for window in stitches.windows(2) {
        cost += window[0]
            .get_end_location()
            .distance_to_location(&window[1].start);
    }
    match end_location {
        None => {}
        Some(loc) => {
            cost += stitches[stitches.len() - 1]
                .get_end_location()
                .distance_to_location(loc);
        }
    }
    // Add cost for each stitch going diagonally
    cost += 2_f64.sqrt() * stitches.len() as f64;
    cost
}

/// Function to verify that a vector of stitches is possible and actually valid. This requires
/// making sure no stitch starts where another ends and making sure that bottom stitches are under
/// top ones.
///
/// # Arguments
///
/// * `stitches`:
///
/// returns: bool
///
pub fn verify_stitches_valid(stitches: &Vec<HalfStitch>) -> bool {
    let mut past_right_stitches: HashSet<Location> = HashSet::new();
    for window in stitches.windows(2) {
        if window[0].get_end_location() == window[1].start {
            return false;
        }

        if !window[0].facing_right {
            let bottom_stitch_location = Location::new(window[0].start.x - 1, window[1].start.y);
            if !past_right_stitches.contains(&bottom_stitch_location) {
                return false;
            }
        }
        if window[0].facing_right {
            past_right_stitches.insert(window[0].start);
        }
    }
    true
}

pub fn make_full_stitch(x: i64, y: i64) -> [HalfStitch; 2] {
    let first: HalfStitch = HalfStitch::new(Location::new(x, y), true);
    let second: HalfStitch = HalfStitch::new(Location::new(x + 1, y), false);
    [first, second]
}

#[cfg(test)]
mod tests {
    use crate::stitch::{get_cost, make_full_stitch, verify_stitches_valid, HalfStitch, Location};

    #[test]
    fn test_end_half_stitch_end_location_right() {
        let test = HalfStitch {
            start: Location { x: 1, y: 0 },
            facing_right: true,
        };
        let result = test.get_end_location();
        assert_eq!(result, Location::new(2, 1));
    }

    #[test]
    fn test_end_half_stitch_end_location_left() {
        let test = HalfStitch {
            start: Location { x: 1, y: 0 },
            facing_right: false,
        };
        let result = test.get_end_location();
        assert_eq!(result, Location::new(0, 1));
    }

    #[test]
    fn test_distance_location_straight() {
        let result = Location::new(0, 0).distance_to_location(&Location::new(1, 0));
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_distance_location_diagonal_one() {
        let result = Location::new(0, 0).distance_to_location(&Location::new(1, 1));
        assert_eq!(result, 2.0_f64.sqrt());
    }

    #[test]
    fn test_distance_location_straight_two() {
        let result = Location::new(0, 0).distance_to_location(&Location::new(2, 0));
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_distance_one_stitch() {
        let test: Vec<HalfStitch> = make_full_stitch(1, 1).to_vec();
        let result = get_cost(&test, &None);
        let expected: f64 = (2.0 * 2.0_f64.sqrt()) + 1.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance_two_stitches() {
        let test = [make_full_stitch(1, 1), make_full_stitch(2, 1)].concat();
        let result = get_cost(&test, &None);
        let expected: f64 = (4.0 * 2.0_f64.sqrt()) + 2.0 + 2_f64.sqrt();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance_three_stitches() {
        let test = [
            make_full_stitch(1, 1),
            make_full_stitch(2, 1),
            make_full_stitch(3, 1),
        ]
        .concat();
        let result = get_cost(&test, &None);
        let expected: f64 = 3.0 * (2.0 * 2.0_f64.sqrt() + 1.0) + (2.0 * 2_f64.sqrt());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance_one_stitch_different_end() {
        let test: Vec<HalfStitch> = make_full_stitch(1, 1).to_vec();
        let result = get_cost(&test, &Some(Location::new(1, 2)));
        let expected: f64 = (2.0 * 2.0_f64.sqrt()) + 1.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance_two_stitches_different_end() {
        let test = [make_full_stitch(1, 1), make_full_stitch(2, 1)].concat();
        let result = get_cost(&test, &Some(Location::new(1, 2)));
        let expected: f64 = (4.0 * 2.0_f64.sqrt()) + 3.0 + 2_f64.sqrt();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_distance_half_series() {
        let test = vec![
            HalfStitch::new(Location::new(1, 1), true),
            HalfStitch::new(Location::new(2, 1), true),
            HalfStitch::new(Location::new(3, 1), true),
            HalfStitch::new(Location::new(4, 1), true),
        ];
        let result = get_cost(&test, &None);
        let expected: f64 = 4.0 * 2.0_f64.sqrt() + 3.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_check_stitches_valid_single_full() {
        let test: Vec<HalfStitch> = make_full_stitch(1, 1).to_vec();
        let result = verify_stitches_valid(&test);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_stitches_valid_single_full_reverse() {
        let mut test: Vec<HalfStitch> = make_full_stitch(1, 1).to_vec();
        test.reverse();
        let result = verify_stitches_valid(&test);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_stitches_valid_two_full() {
        let test: Vec<HalfStitch> = [make_full_stitch(1, 1), make_full_stitch(2, 1)].concat();
        let result = verify_stitches_valid(&test);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_stitches_invalid_location_sequence() {
        let test = vec![
            HalfStitch::new(Location::new(1, 1), true),
            HalfStitch::new(Location::new(2, 2), true),
        ];
        let result = verify_stitches_valid(&test);
        assert_eq!(result, false);
    }
}
