use std::fs::File;
use std::io::{self, BufRead};
use std::env;

pub fn part1() -> io::Result<i32>{
    let mut v_out: Vec<i32> = Vec::new();

    if let Ok(curdir) = env::current_dir() {
        println!("Current directory: {:?}", curdir);
    }
    println!("Reading file...");
    let file = File::open("input/day01.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                let mut d: Vec<char> = Vec::new();
                println!("{}", line_str);
                for c in line_str.chars() {
                    if let Some(_digit) = c.to_digit(10) {
                        d.push(c);
                    }
                }
                let l: String = vec![d[0], d[d.len()-1]].into_iter().collect();
                v_out.push(l.parse::<i32>().unwrap());
            }
            Err(e) => {
                println!("ERROR");
                eprintln!("Error at line {}", e);
            }
        }
    };
    let res = v_out.iter().sum();
    println!("{}", res);
    Ok(res)
}