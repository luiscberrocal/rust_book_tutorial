mod looping;

use clap::{arg, command};
use clap::Parser;

#[derive(Parser)]
#[command(name = "Burger Builder")]
#[command(author = "Luis C. Berrocal")]
#[command(version = "1.0")]
#[command(about = "Build your favorite burger", long_about = None)]
struct Cli {
    #[arg(long, short('s'))]
    style: String,
    #[arg(long("ing"), short)]
    ingredient: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("two: {:?}", cli.style);
    println!("one: {:?}", cli.ingredient);
    looping::loop_testing()
}

