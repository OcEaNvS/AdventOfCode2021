use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    let split_input: Vec<&str>  = input.split("\n").collect();

    for i in 0..split_input.len() {
        let line_split: Vec<&str> = split_input[i].split(" ").collect();

        match line_split[0] {
            "forward" => horizontal += line_split[1].parse::<i32>().unwrap(),
            "down" => depth += line_split[1].parse::<i32>().unwrap(),
            "up" => depth -= line_split[1].parse::<i32>().unwrap(),
            _ => println!("Error"),
        }
    }

    println!("Part 1: {}", depth * horizontal);

    depth = 0;
    horizontal = 0;

    for i in 0..split_input.len() {
        let line_split: Vec<&str> = split_input[i].split(" ").collect();

        match line_split[0] {
            "forward" => {
                    horizontal += line_split[1].parse::<i32>().unwrap();
                    depth += line_split[1].parse::<i32>().unwrap() * aim;
                },
            "down" => aim += line_split[1].parse::<i32>().unwrap(),
            "up" => aim -= line_split[1].parse::<i32>().unwrap(),
            _ => println!("Error"),
        }
    }

    println!("Part 2: {}", depth * horizontal);
}

