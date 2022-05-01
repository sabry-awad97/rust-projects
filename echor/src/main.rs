use clap::{Arg, Command};
fn main() {
    let arg_matches = Command::new("echor")
        .version("0.1.0")
        .author("Dr Sabry <dr.sabry1997@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .allow_invalid_utf8(true)
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();


    let text = arg_matches.values_of_lossy("text").unwrap();

    let omit_newline = arg_matches.is_present("omit_newline");
    print!(
        "{}{}",
        text.join(" "),
        if omit_newline { "" } else { "\n" }
    );
}
