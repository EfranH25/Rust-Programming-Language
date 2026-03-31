use std::{env, error::Error, fs, process};
use minigrep::{search, search_case_insensitive};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // cloning for simplicity > dirty way to fix ownership issues > others better ways to fix
        if args.len() < 3 {
            return Err("ERROR: Not  enough arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case})
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() is one of those rare functions in rust where you need to annotate

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument!:\n{}", err);
        process::exit(1);
    });


    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.file_path);

    run(config).unwrap_or_else(|err| {
        eprintln!("Application arguments error!:\n{}", err);
        process::exit(1);
    });
}
