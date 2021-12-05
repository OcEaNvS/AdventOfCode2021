use std::fs;
use std::io::{self, BufRead};

fn main() {
    let pipes = read_input();
    let mut ocean_floor_p1 = vec![vec![0; 1000]; 1000];
    let mut ocean_floor_p2 = vec![vec![0; 1000]; 1000];
    let mut overlap = 0;

    for pipe in pipes.into_iter() {
        for coord in pipe.coords.into_iter() {
            if !pipe.diagonal {
                ocean_floor_p1[coord.0 as usize][coord.1 as usize] += 1;
            }
            ocean_floor_p2[coord.0 as usize][coord.1 as usize] += 1;
        }
    }

    for row in ocean_floor_p1.into_iter() {
        for cell in row.into_iter() {
            if cell > 1 {
                overlap += 1;
            }
        }
    }

    println!("Part 1: {}", overlap);

    overlap = 0;

    for row in ocean_floor_p2.into_iter() {
        for cell in row.into_iter() {
            if cell > 1 {
                overlap += 1;
            }
        }
    }

    println!("Part 2: {}", overlap);
}

fn read_input() -> Vec<Pipe> {
    let input = fs::File::open("input.txt").expect("File not found");
    let lines = io::BufReader::new(input).lines().map(|l| l.unwrap());
    let mut pipes: Vec<Pipe> = Vec::new();

    for line in lines {
        let coord_vec = line.split(" -> ").collect::<Vec<&str>>();
        let start = coord_vec[0].split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let end = coord_vec[1].split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        pipes.push(Pipe::new((start[0], start[1]), (end[0], end[1])));
    }

    return pipes;
}

struct Pipe {
    coords: Vec<(i32, i32)>,
    diagonal : bool,
}

impl Pipe {
    fn new(start: (i32, i32), end: (i32, i32)) -> Pipe {
        let mut coords = Vec::new();
        let mut x = start.0;
        let mut y = start.1;
        let dx = end.0 - start.0;
        let dy = end.1 - start.1;

        let step = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };

        let sx = dx / step;
        let sy = dy / step;
        
        loop {
            coords.push((x, y));
            if x == end.0 && y == end.1 {
                break;
            }
            x += sx;
            y += sy;
        }

        Pipe {
            coords: coords,
            diagonal: dx.abs() == dy.abs(),
        }
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Pipe {
            coords: Vec::new(),
            diagonal: false,
        }
    }
}