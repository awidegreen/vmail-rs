use std::io::prelude::*;
use std::io;

#[derive(Debug, PartialEq)]
pub enum YesNoAnswer {
    YES,
    NO,
}

pub fn yes_no(message: &str, default: YesNoAnswer) -> YesNoAnswer {
    let mut stdout = io::stdout();

    print!("{} ", message);
    stdout.flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "Yes" | "yes" | "Y" | "y" | "YeS" | "YES" => YesNoAnswer::YES,
        "No" | "NO" | "n" | "N" => YesNoAnswer::NO,
        _ => default,
    }
}
