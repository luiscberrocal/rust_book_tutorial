use clap::{App, Arg};

fn main() {
    let matches = App::new("Burger builder")
        .version("1.0.0")
        .author("Luis C. Berrocal")
        .about("App to build a burger correctly")
        .arg(
            Arg::new("style")
                .long("style")
                .value_name("BURGER_STYLE")
                .help("Type of burger to build")
        )
        .get_matches();

    if let Some(i) = matches.value_of("style"){
        match i {
            "smash" => println!("Got a {} style burger", i),
            "tall" => println!("Got a {} style burger", i),
            _ => println!("Unsupported style!!")
        }
    }
}
