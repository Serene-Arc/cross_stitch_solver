use crate::stitch::{HalfStitch, Location};

#[cfg(test)]
pub fn test_var_valid_sequence_kick() -> Vec<HalfStitch> {
    vec![
        HalfStitch::new(Location::new(1, 1), true),
        HalfStitch::new(Location::new(2, 1), true),
        HalfStitch::new(Location::new(3, 1), false),
        HalfStitch::new(Location::new(3, 2), true),
        HalfStitch::new(Location::new(4, 2), false),
        HalfStitch::new(Location::new(2, 1), false),
    ]
}
