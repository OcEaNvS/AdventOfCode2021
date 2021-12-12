use std::fs;
use std::fmt;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let mut octopus_grid = parse_input(&input);
    let mut part2_octopus_grid = parse_input(&input);
    let mut flash_amount = 0;

    //Part 1
    for i in 0..100{
        increment(&mut octopus_grid);
        for y in 0..octopus_grid.len() {
            for x in 0..octopus_grid[y].len() {
                flash(&mut octopus_grid, (x, y)) ;
            }
        }
        flash_amount += count_flashes(&mut octopus_grid);

        if all_flashed(&octopus_grid) {
            println!("Part 2: {}", i + 1);
            break;
        }
    }
    println!("Part 1: {}", flash_amount);

    //Part 2
    let mut step = 0;
    while !all_flashed(&part2_octopus_grid) {
        increment(&mut part2_octopus_grid);
        for y in 0..part2_octopus_grid.len() {
            for x in 0..part2_octopus_grid[y].len() {
                flash(&mut part2_octopus_grid, (x, y)) ;
            }
        }
        count_flashes(&mut part2_octopus_grid);
        step += 1;
    }

    println!("Part 2: {}", step);
}

fn parse_input(input: &str) -> Vec<Vec<Octopus>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(
                Octopus {
                    power: c.to_digit(10).unwrap(),
                    flashed: false,
                }
            );
        }
        grid.push(row);
    }
    grid
}

fn flash(grid: &mut Vec<Vec<Octopus>>, coords: (usize, usize)) {
    let (x, y) = coords;
    if grid[y][x].power > 9 && !grid[y][x].flashed {
        grid[y][x].flashed = true;
        let mut flash_coords = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                let new_x = x as i32 + i;
                let new_y = y as i32 + j;
                if new_x < 0 || new_y < 0 || new_x >= grid[0].len() as i32 || new_y >= grid.len() as i32 {
                    continue;
                }

                flash_coords.push((new_x as usize, new_y as usize));
                grid[new_y as usize][new_x as usize].power += 1;
            }
        }

        for coord in flash_coords {
            flash(grid, coord);
        }
    }
}

fn increment(grid: &mut Vec<Vec<Octopus>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x].power += 1;
        }
    }
}

fn count_flashes(grid: &mut Vec<Vec<Octopus>>) -> usize {
    let mut flash_count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x].flashed {
                grid[y][x].flashed = false;
                grid[y][x].power = 0;
                flash_count += 1;
            }
        }
    }
    flash_count
}

fn all_flashed(grid: &Vec<Vec<Octopus>>) -> bool {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x].power != 0 {
                return false;
            }
        }
    }
    true
}

struct Octopus {
    power: u32,
    flashed: bool,
}