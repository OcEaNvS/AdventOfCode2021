use std::fs;

fn main() {
    let mut num_arr = fs::read_to_string("input.txt").unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut median = 0;
    let mut mean = num_arr.iter().sum::<i32>() / num_arr.len() as i32;
    let mut distance = 0;
    let mut distance_2 = 0;

    num_arr.sort();

    if (num_arr.len() % 2) == 0 {
        median = (num_arr[num_arr.len() / 2] + num_arr[num_arr.len() / 2 - 1]) / 2;
    } else {
        median = num_arr[num_arr.len() / 2];
    }

    for num in num_arr {
        distance += (num - median).abs();
        distance_2 += (1..=((num - mean).abs())).sum::<i32>();
    }
        
    println!("Part 1: {}", distance);
    println!("Part 2: {}", distance_2);
}