use aoc_23_01::{part1, part2};
use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
struct Args {
    /// Select part (1/2). Otherwise both are run.
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,

    /// Number of the implementation (requires "part" to be set).
    #[arg(short, long, requires("part"))]
    implementation: Option<u8>,

    #[arg(short, long)]
    example: bool,

    #[arg(short, long)]
    verbose: bool,
}

const part_functions: [fn(&str) -> u32; 2] = [part1, part2];

fn get_input_path(part: u8, example: bool) -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    // For some reason, a path to a file can't be "extended" with ".." and then canonicalize it
    // afterwards. Therefore, the file must first be popped from the path.
    path.pop();
    path.push("../..");

    // At this point, path contains the absolute path to the root of the workspace (i.e., path to
    // the advent-of-code repo).

    // Finally, add path to input/example files and canonicalize the final path.
    path.push("23/01/input");

    if example {
        path.push(format!("example{}.txt", part));
    } else {
        path.push(format!("input{}.txt", part));
    }

    path.canonicalize().unwrap()
}

fn main() {
    let args = Args::parse();

    println!("args: {:?}", args);

    match args.part {
        Some(part) if (1..=2).contains(&part) => {
            let path = get_input_path(part, args.example);
            println!("p: {:?}", path);
            let input = fs::read_to_string(path).expect("Cannot read file.");
            if args.verbose {
                println!("Input:\n{}\n", input);
            }

            let result = part_functions[usize::from(part - 1)](&input);
            println!("\n--------------");
            println!("Result Part {}: {}", part, result);
        }
        None => todo!(),
        _ => unreachable!(),
    }
}
