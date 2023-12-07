use std::fs::File;
use std::io::{self, BufRead, BufReader};
use itertools::Itertools;

fn main() -> io::Result<()>{
    // Open the file
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut parsed = reader.lines().map(|line| {
        let line = line.unwrap();
        let (hand, val) = line.split_once(' ').unwrap();
        let val: usize = val.parse().unwrap();
        let hand = Hand::parse(hand);
        (hand, val)
    }).collect::<Vec<_>>();

    parsed.sort_by_key(|p| (p.0.t, p.0.cards.clone()));
    println!("{}", parsed.iter().enumerate().map(|(i, (_, bid))| (i+1)*bid).sum::<usize>());

    Ok(())
}


#[derive(Debug)]
struct Hand {
    cards: Vec<usize>,
    t: usize
}

impl Hand {
    pub fn parse(cards: &str) -> Hand {
        let card_vals = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
        let type_val = [vec![5], vec![1, 4], vec![2, 3], vec![1, 1, 3], vec![1, 2, 2], vec![1, 1, 1, 2], vec![1, 1, 1, 1, 1]];
        let cards: Vec<usize> = cards.chars().map(|c| card_vals.iter().rev().position(|v| v == &c).unwrap()).collect();
        let mut counts: Vec<usize> = cards.iter().counts().into_values().collect();
        counts.sort();
        let t = type_val.iter().rev().position(|v| v == &counts).unwrap();
        Hand {
            cards: cards,
            t: t
        }
    }
}
