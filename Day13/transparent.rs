fn main() {
    let (coordinates, fold) = include_str!("input.txt").split_once("\n\n").unwrap();
    let fold = fold.lines().map(|line| line.trim_start_matches("fold along ")).map(|line| line.split_once("=").unwrap()).map(|(a, b)| (a, b.parse::<i32>().unwrap())).collect::<Vec<_>>();
    let mut coordinates = coordinates.lines().map(|line| line.split_once(",").unwrap()).map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())).collect::<Vec<_>>();
    let mut i = 1;

    println!("Part 1 Coordinates After Fold:");
    for (axis, value) in fold {
        coordinates = coordinates.into_iter().filter_map(|(x, y)| {
            match axis {
                "x" if x == value => None,
                "y" if y == value => None,
                "x" if x > value => Some((value - (x - value), y)),
                "y" if y > value => Some((x, value - (y - value))),
                _ => Some((x, y)),
            }
        }).collect::<Vec<(i32, i32)>>();
        coordinates.sort();
        coordinates.dedup();

        println!("Fold {:2}: {}", i, coordinates.len());
        i += 1;
    }

    let (max_x, max_y) = coordinates.iter().fold((0, 0), |(max_x, max_y), (x, y)| (max_x.max(*x), max_y.max(*y)));
    let mut image = vec![vec![' '; max_x as usize + 1]; max_y as usize + 1];
    for (x, y) in coordinates {
        image[y as usize][x as usize] = '#';
    }

    println!("{}", "\nPart 2 Image:");
    for row in image {
        println!("{}", row.iter().collect::<String>());
    }
}