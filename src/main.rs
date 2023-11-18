use clap::{arg, Arg, ArgMatches, command, Command};

fn main() {
    let matches = Command::new("My Test Program")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .help("A cool file"))
        .arg(Arg::new("num")
            .short('n')
            .long("number")
            .help("Five less than your favorite number"))
        .get_matches();

    let default_file = "input.txt".to_string();
    let myfile: &String = matches.get_one::<String>("file").unwrap_or(&default_file);
    println!("The file passed is: {}", myfile);

    let num_str = matches.get_one::<String>("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
}

fn get_matches_with_macros() -> ArgMatches {
// https://docs.rs/clap/latest/clap/builder/struct.Arg.html#method.num_args
    let matches = command!()
        .arg(
            arg!(
            --style <BURGE_STYLE> "What style of burger you want?"
        )
                .required(true)
        )
        .arg(
            arg!(
                -t --topping <TOPPING> "Topping for your burger"
            ).num_args(0..=10)
        )
        .get_matches();
    matches
}
