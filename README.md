# Connections Score

## Running the program
Rust must be installed on your machine. If you don't have it, [install instructions are here](https://www.rust-lang.org/tools/install).
### With stdin
Run the script with `cargo run`, and paste your results into the command line. Hit enter a couple of times.


### With text files
Paste your NYT Connections results into a .txt file, and invoke the script with
```rust
cargo run -- <path_to_file>
```

You can also score multiple files by simply listing them:
```rust 
cargo run -- <path_1> <path_2> ... <path_n>
```

## Methodology
Each color has a point value based on official difficulty:
- `Purple: 4 =: s_p`
- `Blue: 3 =: s_b`
- `Green: 2 =: s_g`
- `Yellow: 1 =: s_y`

Each game is made up of guesses. Guesses closer to the start score higher.

A Connections game can have a maximum of seven rounds - 3 incorrect guesses, and 4 correct guesses.
Incorrect guesses score `0`, and correct guesses are scored as such:

The score of a round is equal to its color score * a factor `f`. 
`f` is defined such that the highest possible score is `100`.
Rounds further from the start are scaled down - guessing purple on round 1 is a higher score than guessing it on round 7.
We multiply `7 - round number` by the score to get a maximum possible score for our scale factor `f`.
This would be `m := 7*s_p + 6*s_b + 5*s_g + 4*s_y`.
So `f := (100 / m) * (7 - round number)`.
Then `s := color score * f`.

For example, let's say we guess Green correctly on the second round (we use zero-index). 
This means we have
- `green score := 2`
- `f := 100 * (7 - 1) / 60 = 10`
- `round score := 2 * 10 = 20`

Using this methodology, the maximum score is for a game that looks like this:
```
🟪🟪🟪🟪 - 46.67
🟦🟦🟦🟦 - 30.00
🟩🟩🟩🟩 - 16.67
🟨🟨🟨🟨 - 6.67
Total: 100.00
```

The minimum score for a successful game looks like this:
```
🟦🟨🟪🟨 - 0.00
🟩🟦🟨🟪 - 0.00
🟩🟦🟨🟩 - 0.00
🟨🟨🟨🟨 - 6.67
🟩🟩🟩🟩 - 10.00
🟦🟦🟦🟦 - 10.00
🟪🟪🟪🟪 - 6.67
Total: 33.33
```
