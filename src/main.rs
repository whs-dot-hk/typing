use anyhow::Result;
use clap::Parser;
use rand::{thread_rng, Rng};
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    number: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut rng = thread_rng();
    let mut numbers_to_type = String::new();
    for _ in 0..args.number {
        numbers_to_type.push_str(rng.gen_range(0..=9).to_string().as_str());
        numbers_to_type.push(' ');
    }
    println!("{}", numbers_to_type);
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    let user_input: Vec<&str> = user_input
        .trim()
        .split("")
        .filter(|x| !x.is_empty())
        .collect();
    for (i, (a, b)) in numbers_to_type
        .split_whitespace()
        .zip(user_input)
        .enumerate()
    {
        if a == b {
            println!("Digit {}: Correct!", i + 1);
        } else {
            println!("Digit {}: Incorrect. Expected {}, got {}.", i + 1, a, b);
        }
    }
    Ok(())
}
