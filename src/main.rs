use log::debug;
use std::env;
use std::io::{self, BufRead};
use colored::{Colorize, ColoredString};
use std::str;

fn get_args() -> Vec<String>{
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);
    debug!("args = {:?}", args);
    return args;
}

fn color(word: &String, color: usize) -> ColoredString {
    match color {
        0 => {return word.red()}
        1 => {return word.yellow()}
        2 => {return word.blue()}
        3 => {return word.cyan()}
        4 => {return word.purple()}
        5 => {return word.green()}
        _ => {return word.bold()}
    }
}

fn print_line(line: &str, words: &Vec<String>) {
    let mut i = 0;
    'outer: loop {
        if i >= line.len() {
            break
        }
        let sub = &line[i..];
        let mut coloridx = 0;
        for word in words.clone()  {
            if sub.starts_with(word.as_str()) {
                print!("{}", color(&word, coloridx));
                i += word.len();
                continue 'outer;
            }
            coloridx += 1;
        }
        print!("{}", line.chars().nth(i).unwrap());
        i += 1;
    }
    println!("");
}

fn process_stdin() {
    let stdin = io::stdin();
    let words = get_args();

    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();
        print_line(&line, &words);
    }
}

fn main() {
    env_logger::init();
    process_stdin();
}
