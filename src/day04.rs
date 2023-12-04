use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: i32,
    winning_nbs: Vec<i32>,
    nbs: Vec<i32>,
    value: i32
}

impl Card {
    fn new(s: &str) -> Self {
        let parts: Vec<&str> = s.split(":").collect();
        let id = parts[0]
            .trim_start_matches("Card ")
            .parse::<i32>().unwrap();
        
        let numbers: Vec<&str> = parts[1].split("|").collect();
        let winning_nbs: Vec<i32> = numbers[0]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let nbs = numbers[1]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        
        Card {id: id, winning_nbs: winning_nbs, nbs: nbs, value: 0}
    }
    fn compute_value(&mut self) {
        let nbs_set: HashSet<_> = self.nbs
            .clone()
            .into_iter()
            .collect();
        let common_cnt = self.winning_nbs
            .clone()
            .into_iter()
            .filter(|x| nbs_set.contains(x))
            .count();
        if common_cnt == 1 {
            self.value = 1;
        } else if common_cnt > 1 {
            self.value = (2 as i32).pow((common_cnt-1) as u32);
        }
    }
}

pub fn part1() -> io::Result<i32> {
    let success = 0;
    let mut result_p1 = 0;
    let file = File::open("input/day04.txt")?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                println!("{}", line_str);
                let mut c = Card::new(line_str.as_str());
                c.compute_value();
                println!("{:?}", c);
                result_p1 += c.value;
            }
            Err(e) => {
                eprintln!("Error at line {}", e);
            }
        }
    }
    println!("{}", result_p1);
    Ok(success)
}
