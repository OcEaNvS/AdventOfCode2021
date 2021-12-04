use std::fs;
use std::io::{self, BufRead};

fn main() {
    let (numbers, mut boards) = read_input();
    let mut first_score = 0;
    let mut last_score = 0;
    
    for rnd_num in numbers.iter() {
        for i in 0..boards.len() {
            if !boards[i].bingo {
                boards[i].play_num(*rnd_num);
                let won = boards[i].check_bingo();
                if won {
                    if first_score == 0 {
                        first_score = boards[i].score_board(*rnd_num);
                    } else {
                        last_score = boards[i].score_board(*rnd_num);
                    }
                }
            }
        }
    }

    println!("First Win Score: {}", first_score);
    println!("Last Win Score: {}", last_score);
}

fn read_input() -> (Vec<u32>, Vec<Board>) {
    let input = fs::File::open("input.txt").expect("File not found");
    let mut lines = io::BufReader::new(input).lines().map(|l| l.unwrap());

    let rnd_numbers: Vec<u32> = lines.next().unwrap().split(",").map(|s| s.parse::<u32>().unwrap()).collect();

    let mut boards: Vec<Board> = Vec::new();

    let mut current_board : Vec<Vec<u32>> = Vec::new();

    for line in lines {
        if line.is_empty() {
            if current_board.len() > 0 {
                boards.push(Board {
                    cells: current_board, ..Default::default()
                });
            }
            current_board = Vec::new();
        } else {
            let mut row: Vec<u32> = Vec::new();
            for num in line.split(" ") {
                if num.is_empty() { continue; }
                row.push(num.parse::<u32>().unwrap());
            }
            current_board.push(row);
        }
    }

    return (rnd_numbers, boards);
    
}

struct Board {
    cells: Vec<Vec<u32>>,
    hit: Vec<Vec<bool>>,
    bingo: bool,
}

impl Board {
    fn play_num(&mut self, num: u32) {
        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                if self.cells[x][y] == num {
                    self.hit[x][y] = true;
                }
            }
        }
    }

    fn check_bingo(&mut self) -> bool {
        for x in 0..self.hit.len() {
            let mut bingo = true;
            for y in 0..self.hit[x].len() {
                if !self.hit[x][y] {
                    bingo = false;
                    break;
                }
            }
            if bingo {
                self.bingo = true;
                return true;
            }
        }

        for y in 0..self.hit[0].len() {
            let mut bingo = true;
            for x in 0..self.hit.len() {
                if !self.hit[x][y] {
                    bingo = false;
                    break;
                }
            }
            if bingo {
                self.bingo = true;
                return true;
            }
        }
        return false;
    }

    fn score_board(&mut self, curr_num: u32) -> u32 {
        let mut score = 0;
        for x in 0..self.hit.len() {
            for y in 0..self.hit[x].len() {
                if !self.hit[x][y] {
                    score += self.cells[x][y];
                }
            }
        }

        return score * curr_num;
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            cells: Vec::new(),
            hit: vec![vec![false; 5]; 5],
            bingo: false,
        }
    }
}