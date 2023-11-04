use crate::stitch::HalfStitch;
use std::time::Instant;

mod affixed_permutations;
mod csv_reader;
mod stitch;

fn main() {
    let mut best_cost: f64 = f64::MAX;
    let mut best_sequence: Option<Vec<HalfStitch>> = None;

    let now = Instant::now();
    println!("Beginning search...");
    for perm in csv_reader::read_stitches().filter(|p| stitch::verify_stitches_valid(&p)) {
        let calculated_cost = stitch::get_cost(&perm);
        println!("Hit");
        if calculated_cost < best_cost {
            best_cost = calculated_cost;
            best_sequence = Some(perm);
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("Best cost: {}", best_cost);
    match best_sequence {
        None => {}
        Some(s) => {
            for stitch in s {
                println!("{:?}", stitch);
            }
        }
    }
}
