use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::{HashSet, HashMap};

fn main() -> io::Result<()>{
    // Open the file
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut counts: HashMap<i32, i32> = HashMap::new();

    let won: Vec<(i32, i32)> = reader.lines().map(|line| {
        let line = line.unwrap();
        let split1 =line.split_once(':').unwrap(); 
        let card_num: i32 = split1.0.rsplit_once(' ').unwrap().1.parse().unwrap();
        counts.insert(card_num, 1);
        let (winning, mine) = split1.1.split_once('|').unwrap();
        let winning = parse_list(winning);
        let mine = parse_list(mine);
        (card_num, winning.intersection(&mine).count() as i32)
    }).collect();

    won.iter().for_each(|(card_num, count)| {
        (0..counts[&card_num]).for_each(|_| {
            (card_num+1..=card_num+count).for_each(|new_card| {
                counts.insert(new_card, counts[&new_card] + 1);
            })
        })
    });

    println!("{:?}", counts.values().sum::<i32>());


    Ok(())
}

fn parse_list(s: &str) -> HashSet<i32> {
    s.split(' ')
        .filter_map(|x| x.parse().ok())
        .collect()
}
