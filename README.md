# Connections Score


### With stdin
Run the script with `cargo run`, and paste your results into the command line


### With text files
Paste your NYT Connections results into a .txt file, and invoke the script with
```rust
cargo run -- <path_to_file>
```

You can also score multiple files by simply listing them:
```rust 
cargo run -- <path_1> <path_2> ... <path_n>
```
