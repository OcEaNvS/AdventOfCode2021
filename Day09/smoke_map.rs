use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let mut height_vec = Vec::new();
    let mut local_minima: Vec<(usize, usize)> = Vec::new();
    let mut part1_answer: usize = 0;
    let mut basin_size: Vec<usize> = Vec::new();

    for line in input.lines() {
        let height = line.chars().collect::<Vec<char>>();
        height_vec.push(height);
    }

    let mut bool_vec = vec![vec![false; height_vec[0].len()]; height_vec.len()];

    for i in 0..height_vec.len() {
        for j in 0..height_vec[i].len() {
            let current_height = height_vec[i][j].to_digit(10).unwrap();
            if current_height < 9 {
                let index_i = i.checked_sub(1);
                let index_j = j.checked_sub(1);
                if let Some(index) = index_i {
                    if let Some(min) = height_vec.get(index) {
                        if min[j].to_digit(10).unwrap() <= current_height {
                            continue;
                        }
                    } 
                }
                if let Some(min) = height_vec.get(i + 1) {
                    if min[j].to_digit(10).unwrap() <= current_height {
                        continue;
                    }
                } 
                if let Some(min) = height_vec.get(i).unwrap().get(j + 1) {
                    if min.to_digit(10).unwrap() <= current_height {
                        continue;
                    }
                } 
                if let Some(index) = index_j {
                    if let Some(min) = height_vec.get(i).unwrap().get(index) {
                        if min.to_digit(10).unwrap() <= current_height {
                            continue;
                        }
                    }
                }
                local_minima.push((i, j));
            }
        }
    }

    part1_answer += local_minima.len();

    for min in local_minima {
        let min_i = min.0;
        let min_j = min.1;
        let min_height = height_vec[min_i][min_j].to_digit(10).unwrap();
        part1_answer += min_height as usize;
        basin_size.push(find_basin_size(min_i, min_j, &height_vec, &mut bool_vec));
    }
    basin_size.sort_by(|a, b| b.cmp(a));

    println!("Part 1 Result: {}", part1_answer);
    println!("Part 2 Result: {}", (basin_size[0] + 1) * (basin_size[1] + 1) * (basin_size[2] + 1));
}

fn find_basin_size(x: usize, y: usize, height_vec: &Vec<Vec<char>>, bool_vec: &mut Vec<Vec<bool>>) -> usize {
    let current_height = height_vec[x][y].to_digit(10).unwrap();
    let mut count = 0;
    let index_i = x.checked_sub(1);
    let index_j = y.checked_sub(1);
    if let Some(index) = index_i {
        if let Some(min) = height_vec.get(index) {
            if (min[y].to_digit(10).unwrap() > current_height) && (min[y].to_digit(10).unwrap() < 9) && (bool_vec[index][y] == false) {
                bool_vec[index][y] = true;
                count += 1;
                count += find_basin_size(index, y, height_vec, bool_vec);
            }
        } 
    }
    if let Some(min) = height_vec.get(x + 1) {
        if (min[y].to_digit(10).unwrap() > current_height) && (min[y].to_digit(10).unwrap() < 9) && (bool_vec[x + 1][y] == false) {
            bool_vec[x+1][y] = true;
            count += 1;
            count += find_basin_size(x + 1, y, height_vec, bool_vec);
        }
    } 
    if let Some(min) = height_vec.get(x).unwrap().get(y + 1) {
        if (min.to_digit(10).unwrap() > current_height) && (min.to_digit(10).unwrap() < 9) && (bool_vec[x][y + 1] == false) {
            bool_vec[x][y+1] = true;
            count += 1;
            count += find_basin_size(x, y + 1, height_vec, bool_vec);
        }
    } 
    if let Some(index) = index_j {
        if let Some(min) = height_vec.get(x).unwrap().get(index) {
            if (min.to_digit(10).unwrap() > current_height) && (min.to_digit(10).unwrap() < 9) && (bool_vec[x][index] == false) {
                bool_vec[x][index] = true;
                count += 1;
                count += find_basin_size(x, index, height_vec, bool_vec);
            }
        }
    }

    count
}