use std::fs;
use std::path::Path;

fn main() {
    let input = read_file("src/input.txt");
}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    println!("With input:\n{contents}");
    return contents;
}
