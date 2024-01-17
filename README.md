# Connections Score

## Running the program
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
- Purple: 4
- Blue: 3
- Green: 2
- Yellow: 1

Each game is made up of guesses. Guesses closer to the start score higher.

A Connections game can have a maximum of seven rounds - 3 incorrect guesses, and 4 correct guesses.
Incorrect guesses score `0`, and correct guesses are scored as such:


We calculate a `decay factor := 100 / 7`.
We take the `round factor := decay factor * (7 - round number - 1)`.
Then, we calculate `round score = color score * round factor`.

For example, let's say we guess Green correctly on the second round. 
This means we have
- `green score := 2`
- `decay factor := 100 / 7`
- `round factor := decay factor * (7 - round number - 1) = 100 / 7 * (7 - 2 - 1) = 100 / 7 * 6 = 85.71`
- `round score := 2 * 85.71 = 171.43`

Using this methodology, the maximum score is for a game that looks like this:
```
🟪🟪🟪🟪 - 400.00
🟦🟦🟦🟦 - 257.14
🟩🟩🟩🟩 - 142.86
🟨🟨🟨🟨 - 57.14
Total: 857.14
```
The minimum score is for a game that looks like this:
```
🟦🟨🟪🟨 - 0.00
🟩🟦🟨🟪 - 0.00
🟩🟦🟨🟩 - 0.00
🟨🟨🟨🟨 - 57.14
🟩🟩🟩🟩 - 85.71
🟦🟦🟦🟦 - 85.71
🟪🟪🟪🟪 - 57.14
Total: 285.71
```
