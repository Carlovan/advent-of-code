use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{
    // Open the file
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file).lines();

    let t: i64 = reader.next().unwrap().unwrap().split_once(':').unwrap().1.chars().filter(|x| *x!=' ').collect::<String>().parse().unwrap();
    let d: i64 = reader.next().unwrap().unwrap().split_once(':').unwrap().1.chars().filter(|x| *x!=' ').collect::<String>().parse().unwrap();
    let delta = t*t - 4*d;
    let first_winning = (((t as f64) - (delta as f64).sqrt()) / 2_f64).floor() as i64 + 1;
    let output: i64 = (t+1)-2*first_winning;

    println!("{:?}", output);


    Ok(())
}
