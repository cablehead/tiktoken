use std::io::{self, Read};
use tiktoken_rs::cl100k_base;

fn main() {
    // Read input from stdin
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");

    // Initialize the tokenizer
    let bpe = cl100k_base().unwrap();

    // Tokenize the input and count tokens
    let tokens = bpe.encode_with_special_tokens(&buffer);
    let token_count = tokens.len();

    // Print the token count to stdout
    println!("{}", token_count);
}
