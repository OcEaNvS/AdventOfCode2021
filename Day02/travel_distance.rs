fn main() {
    let (depth, horizontal) = include_str!("input.txt").lines().map(|line| line.split_once(" ").unwrap()).fold((0, 0), |(depth, horizontal), (direction, value)| {
        match direction {
            "forward" => (depth, horizontal + value.parse::<i32>().unwrap()),
            "down" => (depth + value.parse::<i32>().unwrap(), horizontal),
            "up" => (depth - value.parse::<i32>().unwrap(), horizontal),
            _ => unreachable!(),
        }
    });

    println!("Part 1: {}", depth * horizontal);

    let (depth, horizontal, _aim) = include_str!("input.txt").lines().map(|line| line.split_once(" ").unwrap()).fold((0, 0, 0), |(depth, horizontal, aim), (direction, value)| {
        match direction {
            "forward" => (depth + value.parse::<i32>().unwrap() * aim, horizontal + value.parse::<i32>().unwrap(), aim),
            "down" => (depth, horizontal, aim + value.parse::<i32>().unwrap()),
            "up" => (depth, horizontal, aim - value.parse::<i32>().unwrap()),
            _ => unreachable!(),
        }
    });

    println!("Part 2: {}", depth * horizontal);
}

