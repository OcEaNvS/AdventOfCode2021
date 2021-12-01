use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let int_arr: Vec<i32> = input.split("\n").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut higher_count = 0;
    let mut curr_num = 0;

    for i in 0..int_arr.len() {
        if i == 0 {
            curr_num = int_arr[i];
            continue;
        }
        if int_arr[i] > curr_num {
            higher_count += 1;
        }
        curr_num = int_arr[i];
    }

    println!("{}", higher_count);
}