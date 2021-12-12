fn main() {
    let points: std::collections::HashMap<char, (usize, usize)> = [('(', (3, 1)), ('[', (57, 2)), ('{', (1197, 3)), ('<', (25137, 4))].iter().cloned().collect();
    let matching_bracket: std::collections::HashMap<char, char> = [(')', '('), (']', '['), ('}', '{'), ('>', '<')].iter().cloned().collect();
    let (lines, mut part1_total, mut part2_total_vec) = (std::fs::read_to_string("input.txt").unwrap().split("\n").map(|x| x.to_string()).collect::<Vec<String>>(), 0, Vec::new());
    'corrupted: for line in lines {
        let mut open_bracket_vector = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => open_bracket_vector.push(c),
                _ => {
                    if open_bracket_vector.last().unwrap() == matching_bracket.get(&c).unwrap() {
                        open_bracket_vector.pop();
                    } else {
                        part1_total += points.get(matching_bracket.get(&c).unwrap()).unwrap().0;
                        continue 'corrupted;
                    }
                }
            }
        }
        part2_total_vec.push(open_bracket_vector.iter().rfold(0, |acc, c| acc * 5 + points.get(c).unwrap().1));
    }
    part2_total_vec.sort();
    println!("Part 1 Score: {}\nPart 2 Score: {}", part1_total, part2_total_vec[part2_total_vec.len() / 2]);
}