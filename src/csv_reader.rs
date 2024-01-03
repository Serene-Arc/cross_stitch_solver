use crate::affixed_permutations::PrefixedPermutations;
use crate::stitch::{make_full_stitch, HalfStitch, Location};
use serde::Deserialize;
use std::io;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct StitchRecord {
    x: i64,
    y: i64,
    modifier: String,
}

impl StitchRecord {
    fn to_stitch(&self) -> [HalfStitch; 2] {
        make_full_stitch(self.x, self.y)
    }
}

pub fn read_stitches_for_solving() -> (Option<HalfStitch>, Vec<HalfStitch>, Option<Location>) {
    let mut first_stitch: Option<HalfStitch> = None;
    let mut last_stitch: Option<HalfStitch> = None;
    let mut inner: Vec<HalfStitch> = Vec::new();

    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.deserialize() {
        let record: StitchRecord = result.expect("read csv");
        let stitches = record.to_stitch();
        if record.modifier.eq("f") {
            first_stitch = Some(stitches[0]);
            inner.push(stitches[1]);
        } else if record.modifier.eq("l") {
            last_stitch = Some(stitches[1]);
            inner.push(stitches[0]);
        } else if record.modifier.eq("lf") || record.modifier.eq("fl") {
            first_stitch = Some(stitches[0]);
            last_stitch = Some(stitches[1]);
        } else {
            inner.push(stitches[0]);
            inner.push(stitches[1]);
        }
    }
    let mut final_loc: Option<Location> = None;
    match last_stitch {
        None => {}
        Some(stitch) => {
            final_loc = Some(stitch.get_end_location());
            inner.push(stitch);
        }
    }

    (first_stitch, inner, final_loc)
}

pub fn read_stitches_for_visualisation() -> Vec<HalfStitch> {
    let mut out = Vec::new();

    let mut reader = csv::Reader::from_reader(io::stdin());
    for result in reader.deserialize() {
        let record: HalfStitch = result.unwrap();
        out.push(record);
    }
    normalize_half_stitch_vec(out)
}

fn normalize_half_stitch_vec(stitches: Vec<HalfStitch>) -> Vec<HalfStitch> {
    if stitches.is_empty() {
        return stitches;
    }

    let min_x = stitches.iter().map(|stitch| stitch.start.x).min().unwrap();
    let min_y = stitches.iter().map(|stitch| stitch.start.y).min().unwrap();

    stitches
        .into_iter()
        .map(|mut stitch| {
            stitch.start.x -= min_x;
            stitch.start.y -= min_y;
            stitch
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::csv_reader::normalize_half_stitch_vec;
    use crate::stitch::{HalfStitch, Location};

    #[test]
    fn test_normalise_single_stitch_positive() {
        let test = vec![HalfStitch::new(Location::new(1, 1), true)];
        let result = normalize_half_stitch_vec(test);
        assert_eq!(result[0].start, Location::new(0, 0))
    }

    #[test]
    fn test_normalise_single_stitch_positive_uneven() {
        let test = vec![HalfStitch::new(Location::new(112, 324), true)];
        let result = normalize_half_stitch_vec(test);
        assert_eq!(result[0].start, Location::new(0, 0))
    }

    #[test]
    fn test_normalise_single_stitch_negative_x() {
        let test = vec![HalfStitch::new(Location::new(-1, 1), true)];
        let result = normalize_half_stitch_vec(test);
        assert_eq!(result[0].start, Location::new(0, 0))
    }

    #[test]
    fn test_normalise_single_stitch_negative_y() {
        let test = vec![HalfStitch::new(Location::new(1, -1), true)];
        let result = normalize_half_stitch_vec(test);
        assert_eq!(result[0].start, Location::new(0, 0))
    }

    #[test]
    fn test_normalise_single_stitch_negative_x_y() {
        let test = vec![HalfStitch::new(Location::new(-1, -1), true)];
        let result = normalize_half_stitch_vec(test);
        assert_eq!(result[0].start, Location::new(0, 0))
    }

    #[test]
    fn test_normalise_two_stitches_positive() {
        let test = vec![
            HalfStitch::new(Location::new(1, 1), true),
            HalfStitch::new(Location::new(2, 1), true),
        ];
        let result = normalize_half_stitch_vec(test);
        assert_eq!(result[0].start, Location::new(0, 0));
        assert_eq!(result[1].start, Location::new(1, 0));
    }

    #[test]
    fn test_normalise_two_stitches_positive_uneven() {
        let test = vec![
            HalfStitch::new(Location::new(438, 121), true),
            HalfStitch::new(Location::new(439, 121), true),
        ];
        let result = normalize_half_stitch_vec(test);
        assert_eq!(result[0].start, Location::new(0, 0));
        assert_eq!(result[1].start, Location::new(1, 0));
    }

    #[test]
    fn test_normalise_two_stitches_negative_x() {
        let test = vec![
            HalfStitch::new(Location::new(-2, 1), true),
            HalfStitch::new(Location::new(-1, 1), true),
        ];
        let result = normalize_half_stitch_vec(test);
        assert_eq!(result[0].start, Location::new(0, 0));
        assert_eq!(result[1].start, Location::new(1, 0));
    }

    #[test]
    fn test_normalise_two_stitches_negative_y() {
        let test = vec![
            HalfStitch::new(Location::new(1, -1), true),
            HalfStitch::new(Location::new(2, -1), true),
        ];
        let result = normalize_half_stitch_vec(test);
        assert_eq!(result[0].start, Location::new(0, 0));
        assert_eq!(result[1].start, Location::new(1, 0));
    }

    #[test]
    fn test_normalise_two_stitches_negative_x_y() {
        let test = vec![
            HalfStitch::new(Location::new(-2, -1), true),
            HalfStitch::new(Location::new(-1, -1), true),
        ];
        let result = normalize_half_stitch_vec(test);
        assert_eq!(result[0].start, Location::new(0, 0));
        assert_eq!(result[1].start, Location::new(1, 0));
    }

    #[test]
    fn test_normalise_three_stitches_triangle() {
        let test = vec![
            HalfStitch::new(Location::new(1, 1), true),
            HalfStitch::new(Location::new(2, 2), true),
            HalfStitch::new(Location::new(3, 1), true),
        ];
        let result = normalize_half_stitch_vec(test);
        assert_eq!(result[0].start, Location::new(0, 0));
        assert_eq!(result[1].start, Location::new(1, 1));
        assert_eq!(result[2].start, Location::new(2, 0));
    }
}
