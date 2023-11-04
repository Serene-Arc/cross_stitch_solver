use crate::affixed_permutations;
use crate::stitch::{make_full_stitch, HalfStitch};
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

pub fn read_stitches() -> affixed_permutations::AffixedPermutations<HalfStitch> {
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
            inner.push(stitches[1]);
        } else if &record.modifier == "lf" || record.modifier.eq("fl") {
            first_stitch = Some(stitches[0]);
            last_stitch = Some(stitches[1]);
        } else {
            inner.push(stitches[0]);
            inner.push(stitches[1]);
        }
    }

    affixed_permutations::AffixedPermutations::new(first_stitch, last_stitch, inner)
}
