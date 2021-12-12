use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input_arr: Vec<&str> = input.split("\n").collect();
    let mut epsilon = String::new();
    let mut gamma = String::new();
    let mut oxygen = String::new();
    let mut carbondioxide = String::new();
    let mut zero = 0;
    let mut one = 0;

    for i in 0..input_arr[0].len() {
        for j in 0..input_arr.len() {
            if input_arr[j].chars().nth(i).unwrap() == '1' {
                one += 1;
            } else {
                zero += 1;
            }
        }
        if one >= zero {
            epsilon.push('1');
            gamma.push('0');
        } else {
            epsilon.push('0');
            gamma.push('1');
        }

        zero = 0;
        one = 0;
    }

    println!("Epsilon: {}", u32::from_str_radix(&epsilon, 2).unwrap());
    println!("Gamma: {}", u32::from_str_radix(&gamma, 2).unwrap());

    let mut oxygen_arr = input_arr.clone();

    for i in 0..oxygen_arr[0].len() {
        for j in 0..oxygen_arr.len() {
            if oxygen_arr[j].chars().nth(i).unwrap() == '1' {
                one += 1;
            } else {
                zero += 1;
            }
        }
        if one >= zero {
            oxygen_arr = oxygen_arr.into_iter().filter(|x| x.chars().nth(i).unwrap() == '1').collect();
        } else {
            oxygen_arr = oxygen_arr.into_iter().filter(|x| x.chars().nth(i).unwrap() == '0').collect();
        }

        one = 0;
        zero = 0;

        if oxygen_arr.len() == 1 {
            oxygen = oxygen_arr[0].to_string();
            break;
        }
    }

    println!("Oxygen: {}", u32::from_str_radix(&oxygen, 2).unwrap());

    let mut carbondioxide_arr = input_arr.clone();

    for i in 0..carbondioxide_arr[0].len() {
        for j in 0..carbondioxide_arr.len() {
            if carbondioxide_arr[j].chars().nth(i).unwrap() == '1' {
                one += 1;
            } else {
                zero += 1;
            }
        }
        if one >= zero {
            carbondioxide_arr = carbondioxide_arr.into_iter().filter(|x| x.chars().nth(i).unwrap() == '0').collect();
        } else {
            carbondioxide_arr = carbondioxide_arr.into_iter().filter(|x| x.chars().nth(i).unwrap() == '1').collect();
        }

        one = 0;
        zero = 0;

        if carbondioxide_arr.len() == 1 {
            carbondioxide = carbondioxide_arr[0].to_string();
            break;
        }
    }

    println!("Carbondioxide: {}", u32::from_str_radix(&carbondioxide, 2).unwrap());
}