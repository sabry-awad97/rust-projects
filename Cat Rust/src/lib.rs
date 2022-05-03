use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let arg_matches = Command::new("catr")
        .version("0.1.0")
        .author("Dr Sabry <dr.sabry1997@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s)")
                .multiple_values(true)
                .default_value("-")
                .allow_invalid_utf8(true),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: arg_matches.values_of_lossy("files").unwrap(),
        number_lines: arg_matches.is_present("number"),
        number_nonblank_lines: arg_matches.is_present("number_nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => {
            let std_input = io::stdin();
            let reader = BufReader::new(std_input);
            Ok(Box::new(reader))
        }
        _ => {
            let filehandle = File::open(filename)?;
            let reader = BufReader::new(filehandle);
            Ok(Box::new(reader))
        }
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => {
                let mut last_number = 0;
                for (line_number, line_text) in reader.lines().enumerate() {
                    let line = line_text?;
                    if config.number_lines {
                        println!("{:6}\t{}", line_number + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_number += 1;
                            println!("{:6}\t{}", last_number, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            },
        }
    }
    Ok(())
}

// chmod 000 cant-touch-this
// cargo run --quiet -- --help
// cargo run -- -n tests/inputs/fox.txt
// cargo run -- -b -n tests/inputs/fox.txt
// cargo run -- fjkdjfads cant-touch-this tests/inputs/fox.txt
// cargo run -- - < tests/inputs/fox.txt