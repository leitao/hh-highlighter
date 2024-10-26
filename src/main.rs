use std::{io::{self, BufRead}, thread, time::Duration};
use colored::{Colorize, ColoredString};
use clap::Parser;

// static CONTEXT: u8 = 0;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, action = clap::ArgAction::Count)]
    context: u8,
    #[arg(short, long, action = clap::ArgAction::Count)]
    wait: u8,
    words: Vec<String>
}

fn color(word: &String, color: usize, context: u8) -> ColoredString {
    let newword = match color {
        0 => {word.red()}
        1 => {word.yellow()}
        2 => {word.blue()}
        3 => {word.cyan()}
        4 => {word.purple()}
        5 => {word.green()}
        _ => {word.bold()}
    };

    match context {
        1 => {return newword.bold();}
        2 => {return newword.bold().on_purple();}
        _ => {return newword;}
    }
}

fn print_line(line: String, words: &Vec<String>, context: u8, wait: u8){
    let char_idxs = line.char_indices();
    let mut skip_count = 0;
    let mut wait_on_next_line = false;
    'outer: for (idx, current_char) in char_idxs {
        if skip_count > 0  {
            // not possible to skip while iterating?! use this hack
            skip_count -= 1;
            continue;
        }
        
        // Line starting a the next element
        let index_substr = &line[idx..];

        for (coloridx, word) in words.iter().enumerate()  {
            if index_substr.starts_with(word.as_str()) {
                skip_count = word.len() - 1;
                print!("{}", color(&word, coloridx, context));
                wait_on_next_line = true;
                continue 'outer;
            }
        }

        // Nothing special found, carry on and print the rest
        print!("{}", current_char);
    }
    println!();
    if wait_on_next_line {
        thread::sleep(Duration::from_secs(wait.into()));
    }
}

fn process_stdin() {
    let stdin = io::stdin();
    let clap_args = Args::parse();
    let words = clap_args.words;
    let context = clap_args.context;
    let wait:u8 = clap_args.wait;

    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();
        print_line(line, &words, context, wait);
    }
}

fn main() {
    env_logger::init();
    process_stdin();
}
