use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;

fn main() -> io::Result<()>{
    // Open the file
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let mut lines = BufReader::new(file).lines();

    let directions: Vec<usize> = lines.next().unwrap().unwrap().chars().map(|c| match c {
        'L' => 0,
        'R' => 1,
        _ => 2
    }).collect();

    lines.next();

    let exp = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    let nodes: HashMap<String, [String; 2]> = lines.map(|line| {
        let line = line.unwrap();
        let (_, [from, to_l, to_r]) = exp.captures(&line).unwrap().extract();
        (from.to_string(), [to_l.to_string(), to_r.to_string()])
    }).collect();

    let total = directions.iter().cycle().scan("AAA", |cur: & mut &str, dir: &usize|  {
        *cur = &nodes.get(*cur).unwrap()[*dir];
        match *cur {
            "ZZZ" => None,
            _ => Some(dir)
        }
    }).count() + 1;
    println!("{}", total);


    Ok(())
}
