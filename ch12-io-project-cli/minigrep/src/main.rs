use std::{env, error::Error, fs, process};

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // cloning for simplicity > dirty way to fix ownership issues > others better ways to fix
        if args.len() < 2 {
            return Err("ERROR: Requires 2 arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n {}", contents);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() is one of those rare functions in rust where you need to annotate

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument!:\n{}", err);
        process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.file_path);

    run(config).unwrap_or_else(|err| {
        println!("Application arguments error!:\n{}", err);
        process::exit(1);
    });
}
