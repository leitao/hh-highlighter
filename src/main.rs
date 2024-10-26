use log::debug;
use std::env;
use std::io::{self, BufRead};
use colored::{Colorize, ColoredString};

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

fn print_line(line: String, words: &Vec<String>){
    let char_idxs = line.char_indices();
    let mut skip_count = 0;

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
                print!("{}", color(&word, coloridx));
                continue 'outer;
            }
        }

        // Nothing special found, carry on and print the rest
        print!("{}", current_char);
    }
     println!();
}

fn process_stdin() {
    let stdin = io::stdin();
    let words = get_args();

    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();
        print_line(line, &words);
    }
}

fn main() {
    env_logger::init();
    process_stdin();
}
