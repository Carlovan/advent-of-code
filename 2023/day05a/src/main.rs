use std::iter::Iterator;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use std::ops::Range;

fn main() -> io::Result<()>{
    // Open the file
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut lines = reader.lines().map(|l| l.unwrap()).peekable();
    let seeds: Vec<i64> = lines.next().unwrap().split_once(' ').unwrap().1.split(' ').map(|x| x.parse().unwrap()).collect();
    lines.next();
    let mut maps = Vec::<AlmanacMap>::new();
    while lines.peek().is_some() {
        maps.push(AlmanacMap::parse(&mut lines));
    }
    println!("{}", seeds.iter().map(|s| maps.iter().enumerate().fold(*s, |acc, (_i, m)| {
        m.map(acc)
    })).min().unwrap());

    Ok(())
}

#[derive(Debug)]
struct AlmanacMap {
    mapping: HashMap<Range<i64>, i64>,
}

impl AlmanacMap {
    pub fn parse<I>(lines: & mut I) -> AlmanacMap
        where I: Iterator<Item = String>
    {
        lines.next();
        AlmanacMap {
            mapping: lines.take_while(|l| l.len()>0).map(|l| {
                    let nums: Vec<i64> = l.split(' ').map(|n| n.parse().unwrap()).collect();
                    ((nums[1]..nums[1] + nums[2]), nums[0])
                }).collect()
        }
    }

    pub fn map(&self, val: i64) -> i64 {
        for (range, to_start) in self.mapping.iter() {
            if range.contains(&val) {
                return val - range.start + to_start
            }
        }
        return val
    }
}
