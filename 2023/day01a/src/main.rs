use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{
    // Open the file
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    println!("{}", reader.lines().map(|line| {
        let line = line.unwrap();
        let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap().to_string().parse::<i32>().unwrap();
        let second_digit = line.chars().rev().find(|c| c.is_digit(10)).unwrap().to_string().parse::<i32>().unwrap();
        first_digit*10 + second_digit
    }).sum::<i32>());

    Ok(())
}
