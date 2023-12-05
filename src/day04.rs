use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: i32,
    winning_nbs: Vec<i32>,
    nbs: Vec<i32>,
    value: i32,
    common_cnt: i32
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
        
        Card {id: id, winning_nbs: winning_nbs, nbs: nbs, value: 0, common_cnt: 0}
    }

    fn compute_common_cnt(&mut self) -> &mut Self {
        let nbs_set: HashSet<_> = self.nbs
            .clone()
            .into_iter()
            .collect();
        let common_cnt = self.winning_nbs
            .clone()
            .into_iter()
            .filter(|x| nbs_set.contains(x))
            .count();
        self.common_cnt = common_cnt as i32;
        self

    }
    fn compute_value(&mut self) -> &mut Self {
        if self.common_cnt == 1 {
            self.value = 1;
        } else if self.common_cnt > 1 {
            self.value = (2 as i32).pow((self.common_cnt-1) as u32);
        }
        self
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
                //println!("{}", line_str);
                let mut c = Card::new(line_str.as_str());
                c.compute_common_cnt().compute_value();
                //println!("{:?}", c);
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

pub fn part2() -> io::Result<i32> {
    let success = 0;
    let mut result_p2 = 0;
    let file = File::open("input/day04.txt")?;
    let reader = io::BufReader::new(file);
    let mut copies: Vec<i32> = (1..=202).collect();
    let mut common_cnt = 0;
    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                let mut c = Card::new(line_str.as_str());
                println!("CARD {}", c.id);
                //println!("COPIES={:?}", copies);
                common_cnt = c.compute_common_cnt().common_cnt;
                println!("COMMON_CNT={}", common_cnt);
                let copies_to_add: Vec<i32> = ((c.id+1)..=(c.id+common_cnt)).collect();
                //println!("COPIES_TO_ADD={:?}",copies_to_add);
                let ub = match c.id {
                    1 => 1,
                    _ => copies.iter().filter(|&x| *x == c.id).count()
                };
                println!("{} is present {} times in COPIES", c.id, ub);
                for _ in 0..ub {
                        copies.extend(copies_to_add.clone());
                    }
                // Flush current c.id occurences to keep a smol-ish vec
                copies.retain(|&item| item != c.id);
                result_p2 += ub as i32;
                }
            Err(e) => {
                eprintln!("Error at line {}", e);
            }
        }
        println!("====");
    }
    println!("{}", result_p2);
    Ok(success)
}
