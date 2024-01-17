use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Copy + paste your Connections results here:");
        let mut results = String::new();
        loop {
            let mut temp = String::new();
            if io::stdin().read_line(&mut temp).is_ok() {
                if temp.trim().is_empty() {
                    break;
                } else {
                    results.push_str(&temp);
                }
            } else {
                break;
            }
        }
        println!("---");
        parse_results(&results);
    } else {
        for filename in args {
            if let Ok(file_contents) = fs::read_to_string(filename) {
                parse_results(&file_contents);
            }
            println!();
        }
    }
}

fn parse_results(results: &str) {
    let max_rounds = 7;
    let decay_factor = 49.0 / max_rounds as f64;
    let scale_factor = 100.0 / 420.0;

    let mut lines = results.lines();
    lines.next(); // discard "Connections"
    let puzzle_num = lines.next().unwrap();
    println!("{puzzle_num}");

    let mut total = 0.0;
    for (idx, line) in lines.enumerate() {
        let round_score = score_round(line, max_rounds - idx, decay_factor, scale_factor);
        println!("{line} - {round_score:.2}");
        total += round_score;
    }
    println!("Total: {total:.2}");
}

fn score_round(line: &str, round: usize, decay_factor: f64, scale_factor: f64) -> f64 {
    let round_factor = scale_factor * decay_factor * round as f64;
    if purple_found(line) {
        4.0 * round_factor
    } else if blue_found(line) {
        3.0 * round_factor
    } else if green_found(line) {
        2.0 * round_factor
    } else if yellow_found(line) {
        1.0 * round_factor
    } else {
        0.0
    }
}

fn purple_found(line: &str) -> bool {
    let purple = '🟪';
    connection_found(line, purple)
}

fn blue_found(line: &str) -> bool {
    let blue = '🟦';
    connection_found(line, blue)
}

fn green_found(line: &str) -> bool {
    let green = '🟩';
    connection_found(line, green)
}

fn yellow_found(line: &str) -> bool {
    let yellow = '🟨';
    connection_found(line, yellow)
}

fn connection_found(line: &str, color: char) -> bool {
    line.chars().all(|c| c == color)
}

// Purple = highest = 64
// Blue = second = 16
// Green = third = 4
// Yellow = lowest = 1
//
// decay factor?
// first line solve = full points
// last line solve = ..... 10% points?
// 7 lines if you use all your mistakes
// 100 / 7
