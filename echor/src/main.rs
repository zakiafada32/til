use clap::{Arg, Command};

fn main() {
    let m = Command::new("echor")
        .author("zaki, afadazaki32@gmail.com")
        .version("0.1.0")
        .about("echo with rust")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .long("omit-newline")
                .help("Omit the trailing newline")
                .num_args(0),
        )
        .get_matches();

    // get raw value of "text" argument
    let text = m
        .get_raw("text")
        .expect("text argument is required")
        .map(|t| t.to_str().expect("text argument is not valid utf-8"))
        .collect::<Vec<&str>>();

    let omit_newline = m.get_flag("omit_newline");
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);
}
