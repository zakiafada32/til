use clap::{Arg, ArgAction, ArgGroup, Command};
use std::{error::Error, io::BufRead};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for file in config.files {
        match open(&file) {
            Ok(data) => {
                if config.number_lines {
                    number_lines(data);
                } else if config.number_nonblank {
                    number_nonblank(data);
                } else {
                    for line in data.lines() {
                        println!("{}", line.unwrap());
                    }
                }
            }
            Err(e) => eprintln!("{}: {}", file, e),
        }
    }
    Ok(())
}

fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(std::io::BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(std::io::BufReader::new(std::fs::File::open(
            file,
        )?))),
    }
}

fn number_lines(data: Box<dyn BufRead>) {
    for (count, line) in data.lines().enumerate() {
        println!("{:6}\t{}", count + 1, line.unwrap());
    }
}

fn number_nonblank(data: Box<dyn BufRead>) {
    let mut count = 1;
    for line in data.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            println!("{:6}\t{}", count, line);
            count += 1;
            continue;
        }

        println!("{}", line);
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Zaki")
        .about("Concatenate files and print on the standard output")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("The files to be concatenated")
                .default_value("-")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number all output lines")
                .conflicts_with("number-nonblank")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number the nonblank output lines")
                .action(ArgAction::SetTrue),
        )
        .group(ArgGroup::new("flags").args(["files"]).multiple(true))
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .map(|s| s.to_string())
            .collect(),
        number_lines: matches.get_flag("number"),
        number_nonblank: matches.get_flag("number-nonblank"),
    })
}
