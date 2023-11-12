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
        } else if &record.modifier == "lf" || record.modifier.eq("fl") {
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
    out
}

pub fn generate_permutations(
    first_stitch: Option<HalfStitch>,
    inner: Vec<HalfStitch>,
) -> PrefixedPermutations<HalfStitch> {
    PrefixedPermutations::new(first_stitch, inner)
}
