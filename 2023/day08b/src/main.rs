use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;
use num::integer::Integer;

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

    let exp = Regex::new(r"([^ ]{3}) = \(([^ ]{3}), ([^ ]{3})\)").unwrap();

    let nodes: HashMap<String, [String; 2]> = lines.map(|line| {
        let line = line.unwrap();
        let (_, [from, to_l, to_r]) = exp.captures(&line).unwrap().extract();
        (from.to_string(), [to_l.to_string(), to_r.to_string()])
    }).collect();

    let startings: Vec<&String> = nodes.keys().filter(|k| k.ends_with("A")).collect();

    // This only works if the paths to each destination are cyclic, which is not specified in the
    // problem description but appears to be true in the input.
    let output: u128 = startings.iter().map(|start| {
        (directions.iter().cycle().scan(*start, |cur: & mut &String, dir: &usize|  {
            *cur = &nodes.get(*cur).unwrap()[*dir];
            if cur.ends_with("Z") {
                None
            } else {
                Some(dir)
            }
        }).count() + 1) as u128
    }).reduce(|a, b| a.lcm(&b)).unwrap();
    println!("{}", output);


    Ok(())
}
