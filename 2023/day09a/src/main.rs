use std::fs::File;
use std::iter::successors;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{
    // Open the file
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let output: i64 = reader.lines().map(|line| {
        let line = line.unwrap();

        let seq: Vec<i64> = line.split(' ').map(|v| v.parse().unwrap()).collect();
        
        let adding: i64 = successors(Some(seq.clone()), |s: &Vec<i64>| -> Option<Vec<i64>> {
            // println!("{:?}", s.iter().zip(s.iter().skip(1)).collect::<Vec<_>>());
            Some(s.iter().zip(s.iter().skip(1)).map(|(a, b)| b - a).collect())
        })
        .skip(1)
        // .inspect(|s| println!("B take_while {:?}", s))
        .take_while(|s| !(s.iter().min() == Some(&0) && s.iter().max() == Some(&0)))
        // .inspect(|s| println!("A take_while {:?}", s))
        .map(|s| *s.last().unwrap())
        .sum();

        seq.last().unwrap() + adding
    })
    // .inspect(|x| println!("{}", x))
    .sum();
    println!("{}", output);

    Ok(())
}
