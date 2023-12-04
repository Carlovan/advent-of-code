use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn main() -> io::Result<()>{
    // Open the file
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let num_re = Regex::new(r"\d+").unwrap();
    let sym_re = Regex::new(r"\*").unwrap();

    let (nums_by_line, (num_idx_by_line, sym_idx_by_line)) : (Vec<HashMap<usize, i32>>, (Vec<HashMap<usize, usize>>, Vec<HashSet<usize>>)) =
        reader.lines().map(|line| {
            let line = line.unwrap();
            let nums = num_re.find_iter(&line).enumerate().map(|(idx, m)| {
                (idx, m.as_str().parse().unwrap())
            }).collect();
            let num_idx = num_re.find_iter(&line).enumerate().flat_map(|(idx, m)| {
                m.range().map(move |pos| (pos, idx))
            }).collect();
            let sym_idx = sym_re.find_iter(&line).map(|m| m.start()).collect();
            (nums, (num_idx, sym_idx))
        }).unzip();
    

    let result: i32 = sym_idx_by_line.into_iter().enumerate()
        .flat_map(|(lidx, syms): (usize, HashSet<usize>)| syms.into_iter().map(|sidx: usize| (lidx, sidx))) // All symb coordinates
        .map(|(lidx, sidx)| {
            (lidx-1..=lidx+1).flat_map(move |lpos| (sidx-1..=sidx+1).map(move |spos| (lpos, spos)))
            // Get existing numbers idx in (lpos, spos), return (lpos, num_idx)
            .filter_map(|(lpos, spos)| num_idx_by_line.get(lpos)
                        .and_then(|l| l.get(&spos))
                        .map(|nidx| (lpos, nidx))
            )
            // Remove duplicated numbers
            .unique()
            // Get number values
            .map(|(lpos, nidx)| nums_by_line[lpos][nidx])
            .collect::<Vec<i32>>() // List of all numbers adjacent to current symbol
        })
        .filter(|adj| adj.len() == 2) // Only keep symbols with 2 adjacent nums
        .map(|adj| adj.iter().product::<i32>())
        .sum();
    println!("{}", result);

    Ok(())
}
