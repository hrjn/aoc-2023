use std::fs::File;
use std::io::{self, BufRead};

pub fn part1() -> io::Result<i32> {
    let success = 0;
    let mut result_p1 = 0;
    let file = File::open("input/day03.txt")?;
    let reader = io::BufReader::new(file);
    let mut mat2d: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                println!("{}", line_str);
                let row: Vec<char> = line_str.chars().collect::<Vec<char>>();
                mat2d.push(row);
            }
            Err(e) => {
                eprintln!("Error at line {}", e);
            }
        }
    }
    let xdim = mat2d[0].len();
    let ydim = mat2d.len();
    let mut buf_str = String::new();
    let mut is_part_nb = 0;
    let mut part_numbers: Vec<i32> = Vec::new();
    for x in 0..xdim {
        for y in 0..ydim {
            if let Some(_val) = mat2d[x][y].to_digit(10) {
                buf_str.push(mat2d[x][y]);
                println!("[{}][{}] = {}", x, y, mat2d[x][y]);
                println!("BUF_STR={}", buf_str);
                // Explore the neighborhood
                for i in -1..=1 {
                    for j in -1..=1 {
                        let ii = x as isize + i;
                        let jj = y as isize + j;
                        if ii >= 0
                            && jj >= 0
                            && ii < xdim as isize 
                            && jj < ydim as isize 
                            && !mat2d[ii as usize][jj as usize].is_digit(10)
                            && mat2d[ii as usize][jj as usize] != '.' {
                                is_part_nb = 1;
                                println!("IS_PART_NB={} (found {})", is_part_nb, mat2d[ii as usize][jj as usize]);
                        }
                    }
                }
            }
            else {
                println!("NaN");
                // Flush buffer
                if !buf_str.is_empty() && is_part_nb == 1 {
                    part_numbers.push(buf_str.parse::<i32>().unwrap());
                }
                buf_str.clear();
                is_part_nb = 0;
            }
            println!("RESULT_P1={}", result_p1);
            println!("IS_PART_NB={}", is_part_nb);
        println!("============");
        }
    }
    result_p1 = part_numbers.iter().sum();
    println!("{:?}", result_p1);
    Ok(success)
}