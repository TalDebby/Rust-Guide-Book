use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
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
        println!("{line}")
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String;
        let file_path: String;
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        if args[1] == "--i" || args[1] == "--ignore-case" {
            if args.len() < 4 {
                return Err("not enough arguments");
            }

            ignore_case = true;
            query = args[2].clone();
            file_path = args[3].clone();
        } else {
            query = args[1].clone();
            file_path = args[2].clone();
        }

        Ok(Config {
            query: query,
            file_path: file_path,
            ignore_case: ignore_case,
        })
    }
}
