use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");
    let (polymer, mappings) = input.split_once("\n\n").unwrap();
    let mut mappings = mappings.lines().map(|line| line.split_once(" -> ").unwrap())
        .map(|(k, v)| ([k.chars().nth(0).unwrap(), k.chars().nth(1).unwrap()], 
            ([k.chars().nth(0).unwrap(), v.chars().next().unwrap()],
            [v.chars().next().unwrap(), k.chars().nth(1).unwrap()]))
        ).collect::<Vec<_>>();

    let mut curr_polymer: Vec<usize> = vec![0; mappings.len()];
    let mut p1_polymer: Vec<usize> = vec![0; mappings.len()];

    mappings.sort_unstable_by_key(|map| map.0);

    let map_to_index = mappings.iter().map(|(_, v)| (mappings.binary_search_by_key(&v.0, |map| map.0).unwrap(), mappings.binary_search_by_key(&v.1, |map| map.0).unwrap())).collect::<Vec<_>>();

    polymer.chars().collect::<Vec<char>>().windows(2).for_each(|pair| {
        curr_polymer[mappings.binary_search_by_key(&pair, |map| &map.0).unwrap()] += 1;
    });

    for i in 0..40 {
        let mut new_polymer: Vec<usize> = vec![0; mappings.len()];
        curr_polymer.iter().enumerate().for_each(|(i, &c)| {
            if c > 0 {
                new_polymer[map_to_index[i].0] += c;
                new_polymer[map_to_index[i].1] += c;
            }
        });
        if i == 9 {p1_polymer = new_polymer.clone();}
        curr_polymer = new_polymer;
    }

    let mut p1_count: Vec<usize> = vec![0; 'Z' as usize - 'A' as usize + 1];
    let mut p2_count: Vec<usize> = vec![0; 'Z' as usize - 'A' as usize + 1];
    p1_count[polymer.chars().last().unwrap() as usize - 'A' as usize] += 1;
    p2_count[polymer.chars().last().unwrap() as usize - 'A' as usize] += 1;

    mappings.iter().zip(p1_polymer).for_each(|(map, number)| {
        p1_count[map.0[0] as usize - 'A' as usize] += number;
    });

    mappings.iter().zip(curr_polymer).for_each(|(map, number)| {
        p2_count[map.0[0] as usize - 'A' as usize] += number;
    });

    let (p1_min, p1_max) = p1_count.iter().filter(|&&c| c > 0).fold((usize::max_value(), 0), |(min, max), &c| (min.min(c), max.max(c)));
    let (p2_min, p2_max) = p2_count.iter().filter(|&&c| c > 0).fold((usize::max_value(), 0), |(min, max), &c| (min.min(c), max.max(c)));

    println!("Part 1: {}\nPart 2: {}", p1_max - p1_min, p2_max - p2_min);
}