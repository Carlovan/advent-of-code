use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{
    let mapping: Vec<(&str, i32)> = [
         (("0", "zero"), 0),
         (("1", "one"), 1),
         (("2", "two"), 2),
         (("3", "three"), 3),
         (("4", "four"), 4),
         (("5", "five"), 5),
         (("6", "six"), 6),
         (("7", "seven"), 7),
         (("8", "eight"), 8),
         (("9", "nine"), 9),
    ].into_iter()
    .flat_map(|t| [(t.0.0, t.1), (t.0.1, t.1)])
    .collect();

    // Open the file
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    println!("{:?}", reader.lines().map(|line| {
        let line = line.unwrap();
        let matches: Vec<(usize, i32)> = mapping.iter().flat_map(|(m, v)| {
            line.match_indices(m).map(|(idx, _)| (idx, *v))
        }).collect();
        matches.iter().min().unwrap().1*10+matches.iter().max().unwrap().1
    }).sum::<i32>());

    Ok(())
}
