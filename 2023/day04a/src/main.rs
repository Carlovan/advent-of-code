use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

fn main() -> io::Result<()>{
    // Open the file
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let result: i32 = reader.lines().map(|line| {
        let line = line.unwrap();
        let (winning, mine) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning = parse_list(winning);
        let mine = parse_list(mine);
        winning.intersection(&mine).copied().collect::<Vec<_>>()
    })
    .map(|won| if won.len() == 0 { 0 } else { 2_i32.pow((won.len() as u32)-1) })
    .sum();
    println!("{:?}", result);


    Ok(())
}

fn parse_list(s: &str) -> HashSet<i32> {
    s.split(' ')
        .filter_map(|x| x.parse().ok())
        .collect()
}
