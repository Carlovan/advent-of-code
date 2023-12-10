use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{
    // Open the file
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file).lines();

    let pairs: Vec<(i32, i32)> = reader.next().unwrap().unwrap().split_once(':').unwrap().1.split(' ').filter_map(|x| x.parse().ok()).zip(
        reader.next().unwrap().unwrap().split_once(':').unwrap().1.split(' ').filter_map(|x| x.parse().ok())).collect();
    let output: i32 =pairs.iter().map(|(t, d)| {
        let delta = t*t - 4*d;
        if delta < 0 {
            return 0;
        }
        let first_winning = (((*t as f64) - (delta as f64).sqrt()) / 2_f64).floor() as i32 + 1;
        (t+1)-2*first_winning
    }).inspect(|v| println!("{}", v)).product();

    println!("{:?}", output);


    Ok(())
}
