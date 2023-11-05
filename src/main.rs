use rayon::prelude::*;
use std::time::Instant;

mod affixed_permutations;
mod csv_reader;
mod stitch;

fn main() {
    brute_force_find();
}

fn brute_force_find() {
    let read_stitches = csv_reader::read_stitches();

    let now = Instant::now();

    let best = csv_reader::generate_permutations(read_stitches.0, read_stitches.1)
        .par_bridge()
        .filter(|p| stitch::verify_stitches_valid(&p))
        .min_by(|s1, s2| {
            stitch::get_cost(s1, &read_stitches.2)
                .total_cmp(&stitch::get_cost(s2, &read_stitches.2))
        });

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    match best {
        None => {
            println!("No best sequence found, uh oh.")
        }
        Some(perm) => {
            let best_cost = stitch::get_cost(&perm, &read_stitches.2);
            println!("Best cost: {}", best_cost);
            for stitch in perm {
                println!("{:?}", stitch);
            }
        }
    }
}
