use crate::stitch::HalfStitch;
use clap::{Parser, Subcommand};
use rayon::prelude::*;
use std::path::PathBuf;
use std::time::Instant;
extern crate piston_window;
use piston_window::*;

mod affixed_permutations;
mod csv_reader;
mod csv_writer;
mod stitch;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Solve {
        #[arg(short, long, default_value = "./output.csv")]
        file: PathBuf,
    },
    Visualise {
        #[arg(short, long, default_value = "./output.gif")]
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Solve { file }) => {
            let sequence = brute_force_find();
            csv_writer::write_solved_sequence_to_file(&sequence, file)
        }
        Some(Commands::Visualise { file }) => {
            let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
                .exit_on_esc(true)
                .build()
                .unwrap();
        }
        None => {}
    }
}

fn brute_force_find() -> Vec<HalfStitch> {
    let read_stitches = csv_reader::read_stitches_for_solving();

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

    match &best {
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
    best.unwrap()
}
