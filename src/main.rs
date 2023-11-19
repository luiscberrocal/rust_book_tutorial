use clap::{arg, command};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json;

mod looping;

#[derive(Serialize)]
struct Burger {
    style: String,
    ingredients: Vec<String>,
}

#[derive(Serialize)]
struct Person {
    name: String,
    age: u32,
    hobbies: Vec<String>,
}

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

    println!("style: {:?}", cli.style);
    for ing in cli.ingredient.iter() {
        println!("Ingredient: {:?}", ing);
    }

    // declaration of the Person struct instance with the name and age fields
    let person = Person {
        name: cli.style,
        age: 28,
        hobbies: cli.ingredient
    };

    // serializes the person struct to JSON using the serde_json library
    let json = serde_json::to_string(&person).expect("Serialization failed");

    // prints the serialized JSON string
    println!("Serialized JSON: {}", json);
    ;
    looping::loop_testing()
}

