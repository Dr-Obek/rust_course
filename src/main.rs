use std::env;
use std::io;
use std::process;
use std::str;

use slug::slugify;
use unidecode::unidecode;

type StringTransformation = fn(&str) -> String;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Please provide only one transformation operation as a command-line argument. \
        Supported operations: lowercase, uppercase, no_spaces, slugify, revert, and remove_diacritics.");
        process::exit(1);
    }

    let operation = &args[1];

    let supported_operations: Vec<(&str, StringTransformation)> = vec![
        ("lowercase", |text: &str| text.to_lowercase()),
        ("uppercase", |text: &str| text.to_uppercase()),
        ("no_spaces", |text: &str| text.replace(" ", "")),
        ("slugify", |text: &str| slugify(text)),
        ("revert", |text: &str| text.chars().rev().collect()),
        ("remove_diacritics", |text: &str| unidecode(text)),
    ];

    if let Some(transform) = supported_operations.iter().find(|(op, _)| op == operation) {
        let (operation_name, transformer) = transform;
        println!("Available text transformation: {}", operation_name);
        loop {
            println!("Enter the text you want to transform (Ctrl+C to exit):");
            let mut input_text = String::new();
            io::stdin().read_line(&mut input_text).expect("Failed to read line");
            let input_text = input_text.trim().to_string();
            let result = transformer(&input_text);
            println!("Transformed text: {}", result);
        }
    } else {
        eprintln!("Invalid operation. Supported operations: lowercase, uppercase, no_spaces, slugify, revert, and remove_diacritics.");
        process::exit(1);
    }
}