use aoc_21_06::{part1, part2};
use clap::Parser;
use log::info;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
struct Args {
    /// Select part (1/2). Otherwise both are run.
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,

    /// Number of the implementation (requires "part" to be set).
    #[arg(short, long, default_value_t = 0, requires("part"))]
    implementation: u8,

    #[arg(short, long)]
    example: bool,

    #[arg(short, long)]
    verbose: bool,
}

const PART_FUNCTIONS: [fn(u8, &str) -> u32; 2] = [part1, part2];

fn get_input_path(part: u8, example: bool) -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    // For some reason, a path to a file can't be "extended" with ".." and then canonicalize it
    // afterwards. Therefore, the file must first be popped from the path.
    path.pop();
    path.push("../..");

    // At this point, path contains the absolute path to the root of the workspace (i.e., path to
    // the advent-of-code repo).

    // Finally, add path to input/example files and canonicalize the final path.
    path.push("21/06/input");

    if example {
        path.push(format!("example{}.txt", part));
    } else {
        path.push(format!("input{}.txt", part));
    }

    path.canonicalize().unwrap()
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        simple_logger::init_with_level(log::Level::Trace).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Error).unwrap();
    }

    info!("args: {:?}", args);

    match args.part {
        Some(part) if (1..=2).contains(&part) => {
            let path = get_input_path(part, args.example);
            info!("Input path: {:?}", path);
            let input = fs::read_to_string(path).expect("Cannot read file.");
            info!("Input:\n{}\n", input);

            let result = PART_FUNCTIONS[usize::from(part - 1)](args.implementation, &input);
            println!("\n--------------");
            println!(
                "Result Part {} Implementation {}: {}",
                part, args.implementation, result
            );
        }
        None => todo!(),
        _ => unreachable!(),
    }
}
