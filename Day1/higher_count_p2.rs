use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let int_arr: Vec<i32> = input.split("\n").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut summed_arr: Vec<i32> = Vec::new();
    let mut higher_count = 0;
    let mut curr_num = 0;

    for i in 2..int_arr.len() {
        summed_arr.push(int_arr[i] + int_arr[i-1] + int_arr[i-2]);
    }

    println!("{}", summed_arr[0]);

    for i in 0..summed_arr.len() {
        if i == 0 {
            curr_num = summed_arr[i];
            continue;
        }
        if summed_arr[i] > curr_num {
            higher_count += 1;
        }
        curr_num = summed_arr[i];
    }

    println!("{}", higher_count);
}