use std::fs;
use std::convert::TryInto;

fn main() {
    let fish_arr = fs::read_to_string("input.txt").unwrap().split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut count_arr = vec![0; 9];
    let mut day = 0;

    for i in 0..=8 {
        count_arr[i] = fish_arr.iter().filter(|&x| *x == i.try_into().unwrap()).count();
    }

    while day < 256 {
        count_arr.rotate_left(1);
        count_arr[6] += count_arr[8];

        if day == 79 {
            println!("Part 1 Fish Count: {}", count_arr.iter().sum::<usize>());
        }

        day += 1;
    }

    println!("Part 2 Fish Count: {}", count_arr.iter().sum::<usize>());
}