//extern crate clipboard;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io;

fn read_file_to_vector(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Could not parse line");
        lines.push(line);
    }

    return lines;
}

fn main() {
    let lines = read_file_to_vector("my_file.txt");

    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Failed to initialize clipboard");
    let mut index = 0;
    let mut formatted_line = String::new();

    loop {
        if index >= lines.len() {
            // Erase clipboard contents and exit the loop
            ctx.set_contents(String::new()).expect("Failed to set clipboard contents");
            break;
        }

        
        formatted_line = format!("my name is {}", lines[index]);
        ctx.set_contents(formatted_line.clone()).expect("Failed to set clipboard contents");
        println!("{}", lines[index]);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // Check if Enter or some key was pressed
        if input.trim().is_empty() {
            index += 1;
        }
    }
}
