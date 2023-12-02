use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;

fn main() -> io::Result<()>{
    // Open the file
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"^Game (\d+):(.+)$").unwrap();
    let maxes = HashMap::<String, i32>::from([
                                           ("red".to_string(), 12),
                                           ("green".to_string(), 13),
                                           ("blue".to_string(), 14),
    ]);

    println!("{}", reader.lines().into_iter().map(|line| {
        let l = line.unwrap();
        let (_, [id, cubes]) = re.captures(&l).unwrap().extract();
        let id: i32 = id.parse().unwrap();

        let parsed = cubes.split(';')
            .flat_map(|x| x.split(',')
                      .map(|x| x.trim().split_once(' ').unwrap()))
                      .map(|x| (x.1.to_string(), x.0.parse::<i32>().unwrap()))
            .fold(HashMap::<String, i32>::new(), |mut map, (key, value)| {
                map.entry(key)
                    .and_modify(|existing| *existing = (*existing).max(value))
                    .or_insert(value);
                map
            });
        (id, parsed)
    }).filter_map(|(id, parsed)| {
        if parsed.into_iter().all(|(color, amount)| amount <= maxes[&color]) {
            Some(id)
        } else {
            None
        }
    }).sum::<i32>());


    Ok(())
}
