use crate::stitch::{HalfStitch, Location};

#[cfg(test)]
pub fn test_var_valid_sequence_kick() -> Vec<HalfStitch> {
    vec![
        HalfStitch::new(Location::new(1, 1), true),  // ends 2,2
        HalfStitch::new(Location::new(2, 1), true),  // ends 3,2
        HalfStitch::new(Location::new(3, 1), false), // ends 2,2
        HalfStitch::new(Location::new(3, 2), true),  // ends 4,3
        HalfStitch::new(Location::new(4, 2), false), // ends 3,3
        HalfStitch::new(Location::new(2, 1), false), // ends 1,2
    ]
}
