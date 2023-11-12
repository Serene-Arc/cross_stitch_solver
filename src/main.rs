use crate::stitch::HalfStitch;
use clap::{Parser, Subcommand};
use gif::Encoder;
use gif::Frame;
use gif::Repeat;
use image::imageops::flip_vertical;
use image::RgbaImage;
use imageproc::drawing::draw_line_segment_mut;
use rayon::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;

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
        output_file: PathBuf,
    },
    Visualise {
        #[arg(short, long, default_value = "./output.gif")]
        output_file: PathBuf,
    },
}

fn calculate_offset(number: i64) -> i64 {
    number * 10 + 5
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Solve { output_file }) => {
            let sequence = brute_force_find();
            csv_writer::write_solved_sequence_to_file(&sequence, output_file)
        }
        Some(Commands::Visualise { output_file }) => {
            let stitches = csv_reader::read_stitches_for_visualisation();
            let max_x = (&stitches)
                .iter()
                .map(|s| s.get_end_location().x)
                .max()
                .unwrap();
            let max_y = (&stitches)
                .iter()
                .map(|s| s.get_end_location().y)
                .max()
                .unwrap();
            let width = (calculate_offset(max_x) + 5) as u32;
            let height = (calculate_offset(max_y) + 5) as u32;
            let black = image::Rgba([0, 0, 0, 255]);
            let mut img =
                RgbaImage::from_fn(width, height, |_, _| image::Rgba([255, 255, 255, 255]));

            // Make the stitch points black
            for stitch in &stitches {
                img.put_pixel(
                    calculate_offset(stitch.start.x) as u32,
                    calculate_offset(stitch.start.y) as u32,
                    black,
                );
                img.put_pixel(
                    calculate_offset(stitch.get_end_location().x) as u32,
                    calculate_offset(stitch.get_end_location().y) as u32,
                    black,
                );
            }

            let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
            let mut encoder = Encoder::new(
                File::create(output_file).unwrap(),
                width as u16,
                height as u16,
                color_map,
            )
            .unwrap();
            encoder.set_repeat(Repeat::Infinite).unwrap();

            let starting_frame = Frame::from_rgba_speed(
                width as u16,
                height as u16,
                &mut flip_vertical(&img).as_raw().clone(),
                30,
            );
            encoder.write_frame(&starting_frame).unwrap();

            for stitch in stitches {
                draw_line_segment_mut(
                    &mut img,
                    (
                        calculate_offset(stitch.start.x) as f32,
                        calculate_offset(stitch.start.y) as f32,
                    ),
                    (
                        calculate_offset(stitch.get_end_location().x) as f32,
                        calculate_offset(stitch.get_end_location().y) as f32,
                    ),
                    black,
                );
                let mut frame = Frame::from_rgba(
                    width as u16,
                    height as u16,
                    &mut flip_vertical(&img).as_raw().clone(),
                );
                frame.delay = 50;
                encoder.write_frame(&frame).unwrap();
            }
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
