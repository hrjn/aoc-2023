use std::fs::File;
use std::io::{self, BufRead};
use std::vec;



pub fn part1() -> io::Result<i32> {
    let success = 0;
    let mut result_p1 = 0;
    let file = File::open("input/day03.txt")?;
    let reader = io::BufReader::new(file);
    let mut mat2d: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                //println!("{}", line_str);
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
                //println!("[{}][{}] = {}", x, y, mat2d[x][y]);
                //println!("BUF_STR={}", buf_str);
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
                                //println!("IS_PART_NB={} (found {})", is_part_nb, mat2d[ii as usize][jj as usize]);
                        }
                    }
                }
            }
            else {
                //println!("NaN");
                // Flush buffer
                if !buf_str.is_empty() && is_part_nb == 1 {
                    part_numbers.push(buf_str.parse::<i32>().unwrap());
                }
                buf_str.clear();
                is_part_nb = 0;
            }
            //println!("RESULT_P1={}", result_p1);
            //println!("IS_PART_NB={}", is_part_nb);
        //println!("============");
        }
    }
    result_p1 = part_numbers.iter().sum();
    println!("{:?}", result_p1);
    Ok(success)
}

pub fn part2() -> io::Result<i32>{
    let success = 0;
    let mut result_p2 = 0;
    let file = File::open("input/day03.txt")?;
    let reader = io::BufReader::new(file);
    let mut mat2d: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                //println!("{}", line_str);
                let row: Vec<char> = line_str.chars().collect::<Vec<char>>();
                mat2d.push(row);
            }
            Err(e) => {
                eprintln!("Error at line {}", e);
            }
        }
    }
    println!("=========");
    let xdim = mat2d[0].len();
    let ydim = mat2d.len();
    let mut pn_str = String::new();
    let mut pn_buf: Vec<i32> = Vec::new();
    let mut visited: Vec<(isize, isize)> = Vec::new();
    for x in 0..xdim {
        for y in 0..ydim {
            if mat2d[x][y] == '*' {
                println!("* at [{}][{}]", x, y);
                // Explore neighborhood
                // TODO create an explored list from left/right searches and omit (ii,jj) already visited
                visited.clear();
                'outer: for i in -1..=1 {
                    for j in -1..=1{
                        let ii = x as isize + i;
                        let jj = y as isize + j;
                        let pn_c = mat2d[ii as usize][jj as usize];
                        println!("Exploring [{}][{}]...", ii, jj);
                        if ii >= 0
                            && jj >= 0
                            && ii < xdim as isize
                            && jj < ydim as isize
                            && pn_c.is_digit(10)
                            && !visited.contains(&(ii, jj)) {
                                visited.push((ii,jj));
                                println!("VISITED={:?}", visited);
                                pn_str.push(pn_c);
                                println!("Found {} [{}][{}]", pn_c, ii, jj);
                                println!("Looking left...");
                                let mut k = jj;
                                while k-1 >= 0 {
                                    k -= 1;
                                    visited.push((ii,k));
                                    let cc = mat2d[ii as usize][k as usize];
                                    if cc.is_digit(10) {
                                        pn_str.push(cc);
                                        println!("{}", pn_str);
                                    } else {
                                        break;
                                    }
                                }
                                // Reverse string before searching forward
                                pn_str = pn_str.chars().rev().collect();
                                println!("{}", pn_str);
                                println!("Looking right...");
                                // Reset k back to initial position
                                k = jj;
                                while k+1 < ydim as isize {
                                    k += 1;
                                    visited.push((ii,k));
                                    let cc = mat2d[ii as usize][k as usize];
                                    if cc.is_digit(10) {
                                        pn_str.push(cc);
                                        println!("{}", pn_str);
                                    } else {
                                        break;
                                    }
                                }
                                pn_buf.push(pn_str.parse::<i32>().unwrap());
                                pn_str.clear();
                                println!("{:?}", pn_buf);
                                if pn_buf.len() == 2 {
                                    result_p2 += pn_buf.iter().product::<i32>();
                                    pn_buf.clear();
                                    break 'outer;
                                }
                        }
                    }
                }
                pn_buf.clear();
                println!("=========");
            }
        }
    }
    println!("{}", result_p2);
    Ok(success)

}