use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Copy + paste your Connections results here:");
        let results = read_from_stdin();
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

fn read_from_stdin() -> String {
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
    results
}

fn parse_results(results: &str) {
    let mut lines = results.lines();
    lines.next(); // discard "Connections"
    let puzzle_num = lines.next().unwrap();
    println!("{puzzle_num}");

    let mut total = 0.0;
    for (idx, line) in lines.enumerate() {
        let round_score = score_round(line, idx);
        println!("{line} - {round_score:.2}");
        total += round_score;
    }
    println!("Total: {total:.2}");
}

fn score_round(line: &str, round: usize) -> f64 {
    let scores = [4.0, 3.0, 2.0, 1.0];

    let max_score = 7.0 * scores[0] + 6.0 * scores[1] + 5.0 * scores[2] + 4.0 * scores[3];
    let round_factor = (100.0 * (7.0 - round as f64)) / max_score;

    if purple_found(line) {
        scores[0] * round_factor
    } else if blue_found(line) {
        scores[1] * round_factor
    } else if green_found(line) {
        scores[2] * round_factor
    } else if yellow_found(line) {
        scores[3] * round_factor
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
