use pathfinding::prelude::dijkstra;
use std::fs;

const NEIGHBOURS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let cave = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let shortest_path_p1: (Vec<(i32, i32)>, u32) = dijkstra(&(0, 0), |(x, y)| {
        NEIGHBOURS.iter()
            .map(|(dx, dy)| {
                cave.get((y + dy) as usize).and_then(|row| row.get((x + dx) as usize)).map(|c| ((x + dx, y + dy), *c as u32))
            }).flatten().collect::<Vec<_>>()
    }, |&node| node == (99, 99)).unwrap();

    println!("Part 1: {}", shortest_path_p1.1);

    let shortest_path_p2: (Vec<(i32, i32)>, u32) = dijkstra(&(0, 0), |(x, y)| {
        NEIGHBOURS.iter()
            .map(|(dx, dy)| ((x + dx) as usize, (y + dy) as usize))
            .filter(|(x, y)| x / 5 < cave.len() && y / 5 < cave.len())
            .map(|(x, y)| {
                cave.get(y % cave.len())
                .and_then(|row| row.get(x % row.len()))
                .map(|c| ((x as i32, y as i32), ((*c as usize + x / cave.len() + y / cave.len() - 1) % 9 + 1) as u32))
            }).flatten().collect::<Vec<_>>()
    }, |&node| node == (499, 499)).unwrap();

    println!("Part 2: {}", shortest_path_p2.1);
}

