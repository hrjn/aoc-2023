use std::fs::File;
use std::io::{self, BufRead};

pub fn part1() -> io::Result<i32>{
    let mut v_out: Vec<i32> = Vec::new();

    let file = File::open("input/day01.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                let mut d: Vec<char> = Vec::new();
                for c in line_str.chars() {
                    if let Some(_digit) = c.to_digit(10) {
                        d.push(c);
                    }
                }
                let l: String = vec![d[0], d[d.len()-1]].into_iter().collect();
                v_out.push(l.parse::<i32>().unwrap());
            }
            Err(e) => {
                eprintln!("Error at line {}", e);
            }
        }
    };
    let res = v_out.iter().sum();
    //println!("{}", res);
    Ok(res)
}

pub fn part2() -> io::Result<i32>{
    let lut: Vec<(&str, &str)> = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ];
    let mut v_out: Vec<i32> = Vec::new();
     let file = File::open("input/day01.txt")?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                println!("{}", line_str);
                let mut d: Vec<char> = Vec::new();
                let mut cur_word = String::new();
                for c in line_str.chars(){
                    if let Some(_digit) = c.to_digit(10){
                        d.push(c);
                    }
                    else {
                        cur_word.push(c);
                        println!("curword = {}", cur_word);
                        for (word, digit) in &lut{
                            if cur_word.contains(*word){
                                d.push(digit.chars().next().unwrap());
                                if let Some(fc) = cur_word.chars().last() {
                                    cur_word = fc.to_string();
                                }
                            }
                        }
                    }
                }
                let l: String = vec![d[0], d[d.len()-1]].into_iter().collect();
                println!("{:?}", l);
                v_out.push(l.parse::<i32>().unwrap());
            }
            Err(e) => {
            eprintln!("Error at line {}", e);
        }
        }
    }
    let res = v_out.iter().sum();
    println!("{}", res);
    Ok(res)
}