use crate::affixed_permutations::PrefixedPermutations;
use crate::closest_n_permutation::ClosestNElementsIterator;
use crate::stitch::{HalfStitch, Location};
use clap::{Parser, Subcommand};
use factorial::Factorial;
use gif::Encoder;
use gif::Frame;
use gif::Repeat;
use image::imageops::flip_vertical;
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_line_segment_mut;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;

mod affixed_permutations;
mod closest_n_permutation;
mod csv_reader;
mod csv_writer;
mod stitch;
#[cfg(test)]
mod test_sequences;

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
        #[arg(short, long, default_value = "closest-n")]
        mode: String,
        #[arg(short, long, default_value_t = 3)]
        closest_n: usize,
    },
    Visualise {
        #[arg(short, long, default_value = "./output.gif")]
        output_file: PathBuf,
    },
    Calculate {},
}

fn calculate_offset(number: i64) -> i64 {
    number * 10 + 5
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Solve {
            output_file,
            mode,
            closest_n,
        }) => match mode.as_str() {
            "brute-force" => {
                let sequence = brute_force_find();
                csv_writer::write_solved_sequence_to_file(&sequence, output_file)
            }
            "closest-n" => {
                let sequence = closest_n_find(*closest_n);
                csv_writer::write_solved_sequence_to_file(&sequence, output_file)
            }
            _ => println!("Solver mode '{}' not recognised", mode),
        },
        Some(Commands::Visualise { output_file }) => {
            // Setup some starting variables for the canvas
            let stitches = csv_reader::read_stitches_for_visualisation();
            let max_x = stitches
                .iter()
                .map(|s| s.get_end_location().x)
                .max()
                .unwrap();
            let max_y = stitches
                .iter()
                .map(|s| s.get_end_location().y)
                .max()
                .unwrap();
            let width = (calculate_offset(max_x) + 5) as u32;
            let height = (calculate_offset(max_y) + 5) as u32;
            let black = Rgba([0, 0, 0, 255]);

            // Make the background/first frame
            let mut img = make_background_with_stitch_points(&stitches, width, height, black);

            // Set up the gif encoder
            let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
            let mut encoder = Encoder::new(
                File::create(output_file).unwrap(),
                width as u16,
                height as u16,
                color_map,
            )
            .unwrap();
            encoder.set_repeat(Repeat::Infinite).unwrap();

            // Encode the first frame
            let starting_frame = Frame::from_rgba_speed(
                width as u16,
                height as u16,
                &mut flip_vertical(&img).as_raw().clone(),
                30,
            );
            encoder.write_frame(&starting_frame).unwrap();

            // Animate the stitches being drawn
            draw_stitches(stitches, width, height, black, &mut img, &mut encoder);
        }
        Some(Commands::Calculate {}) => {
            let stitches = csv_reader::read_stitches_for_visualisation();
            let cost = stitch::get_cost(&stitches, &None);
            println!("Total Cost: {}", cost);
        }
        None => {}
    }
}

fn draw_stitches(
    stitches: Vec<HalfStitch>,
    width: u32,
    height: u32,
    black: Rgba<u8>,
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    encoder: &mut Encoder<File>,
) {
    for stitch in stitches {
        draw_line_segment_mut(
            img,
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
            &mut flip_vertical(img).as_raw().clone(),
        );
        frame.delay = 50;
        encoder.write_frame(&frame).unwrap();
    }
}

fn make_background_with_stitch_points(
    stitches: &Vec<HalfStitch>,
    width: u32,
    height: u32,
    black: Rgba<u8>,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = RgbaImage::from_fn(width, height, |_, _| Rgba([255, 255, 255, 255]));

    // Make the stitch points black
    for stitch in stitches {
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
    img
}

fn brute_force_find() -> Vec<HalfStitch> {
    let read_stitches = csv_reader::read_stitches_for_solving();
    let number_of_stitches: u128 = read_stitches.1.len() as u128;

    let now = Instant::now();

    let first_stitch = read_stitches.0;
    let inner = read_stitches.1;
    let best = PrefixedPermutations::new(first_stitch, inner)
        .par_bridge()
        .progress_count(number_of_stitches.factorial() as u64)
        .with_style(
            ProgressStyle::with_template("[{elapsed_precise}] {wide_bar} {human_pos}/{human_len} ({percent}%) [{eta_precise}]")
                .unwrap(),
        )
        .min_by(|s1, s2| {
            stitch::get_cost(s1, &read_stitches.2)
                .total_cmp(&stitch::get_cost(s2, &read_stitches.2))
        });

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    process_best_result(&read_stitches.2, best)
}

fn closest_n_find(n_value: usize) -> Vec<HalfStitch> {
    let read_stitches = csv_reader::read_stitches_for_solving();

    let now = Instant::now();
    let iterator_length: u64;
    let iterator: Box<dyn Iterator<Item = Vec<HalfStitch>> + Send>;
    match read_stitches.0 {
        None => {
            iterator_length = (read_stitches.1.len() * n_value * read_stitches.1.len()) as u64;
            iterator = Box::new(
                read_stitches
                    .1
                    .iter()
                    .map(|s| {
                        (
                            s,
                            read_stitches
                                .1
                                .iter()
                                .filter(|s2| *s2 != s)
                                .copied()
                                .collect::<Vec<HalfStitch>>(),
                        )
                    })
                    .flat_map(|(start_stitch, stitch_vec)| {
                        ClosestNElementsIterator::new(*start_stitch, stitch_vec, n_value)
                    }),
            );
        }
        Some(first_location) => {
            iterator = Box::new(ClosestNElementsIterator::new(
                first_location,
                read_stitches.1.clone(),
                n_value,
            ));
            iterator_length = n_value.pow(read_stitches.1.len() as u32) as u64;
        }
    }
    let cost_data = read_stitches.2;
    let best = iterator
        .par_bridge()
        .progress_count(iterator_length)
        .with_style(
            ProgressStyle::with_template("[{elapsed_precise}] {wide_bar} {human_pos}/{human_len} ({percent}%) [{eta_precise}]")
                .unwrap(),
        )
        .min_by(|s1, s2| {
            stitch::get_cost(s1, &cost_data)
                .total_cmp(&stitch::get_cost(s2, &read_stitches.2))
        });
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    process_best_result(&read_stitches.2, best)
}

fn process_best_result(
    final_location: &Option<Location>,
    best: Option<Vec<HalfStitch>>,
) -> Vec<HalfStitch> {
    match &best {
        None => {
            println!("No best sequence found, uh oh.")
        }
        Some(perm) => {
            let best_cost = stitch::get_cost(perm, final_location);
            println!("Best cost: {}", best_cost);
            for stitch in perm {
                println!("{:?}", stitch);
            }
        }
    }
    best.unwrap()
}
