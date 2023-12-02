use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct GameSet {
    red: i32,
    blue: i32,
    green: i32,
}

impl GameSet {
    fn new() -> Self {
        GameSet { red: 0, blue: 0, green: 0 }
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<GameSet>,
    is_possible: bool,
    power: i32
}

impl Game {
    fn max_values(&self) -> (i32, i32, i32) {
        let mut max_red = std::i32::MIN;
        let mut max_blue = std::i32::MIN;
        let mut max_green = std::i32::MIN;

        for game_set in &self.sets {
            max_red = max_red.max(game_set.red);
            max_blue = max_blue.max(game_set.blue);
            max_green = max_green.max(game_set.green);
        }

        (max_red, max_blue, max_green)
    }
    fn set_power(&mut self) {
        let (mr, mg, mb) = self.max_values();
        self.power = mr*mg*mb;
    }
}

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

pub fn part12() -> io::Result<i32>{
    let success = 0;
    let mut result_p1 = 0;
    let mut result_p2 = 0;
    let file = File::open("input/day02.txt")?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                let s: Vec<&str> = line_str.split(':').collect();
                // Game instanciation
                let mut game = Game {id: 0, sets: Vec::<GameSet>::new(), is_possible: true, power: 0};
                game.id = s[0].split(" ").last().unwrap().parse::<i32>().unwrap();
                let game_sets_str: Vec<&str> = s[1].split(";").collect();
                for gs in game_sets_str {
                    let mut gameset = GameSet::new();
                    for t in gs.split(',') {
                        let parts: Vec<&str> = t.trim().split_whitespace().collect();
                        if parts.len() == 2 {
                            let cnt_str = parts[0];
                            let color = parts[1];
                            if let Ok(cnt) = cnt_str.parse::<i32>(){
                                match color {
                                    "red" => gameset.red += cnt,
                                    "blue" => gameset.blue += cnt,
                                    "green" => gameset.green += cnt,
                                    _ => println!("NOPE"), // UGLY AF
                                }
                            }
                            else {
                                println!("NOPE");
                            }
                        }
                        else {
                            println!("Invalid format");
                        }
                    }
                    if gameset.red > MAX_RED || gameset.green > MAX_GREEN || gameset.blue > MAX_BLUE {
                        game.is_possible = false;
                    }
                    game.sets.push(gameset);
                }
                if game.is_possible == true {
                    result_p1 += game.id;
                }
                game.set_power();
                result_p2 += game.power;


            }
            Err(e) => {
                eprintln!("Error at line {}", e);
            }
        }
    }
    println!("{}", result_p1);
    println!("{}", result_p2);
    Ok(success)
}

