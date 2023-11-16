use clap::{arg, command};

fn main() {
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
    if let Some(style) = matches.get_one::<String>("style") {
        println!("Value for name: {style}");
    }
}
