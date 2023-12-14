use std::fs;

fn hash_row(row: &Vec<char>) -> usize {
    let mut hash = 0;
    for (i, c) in row.iter().enumerate() {
        if *c == '#' {
            hash += 1 << i
        }
    }
    hash
}

fn check_palindrome(mirror: &[usize]) -> bool {
    for i in 0..mirror.len() / 2 {
        if mirror[i] != mirror[mirror.len() - 1 - i] {
            return false;
        }
    }
    true
}

fn reflection(mountain: &Vec<Vec<char>>) -> Option<usize> {
    let mut simpler_map: Vec<usize> = Vec::new();
    for line in mountain {
        simpler_map.push(hash_row(line));
    }
    let end = simpler_map.len();
    let mut biggest_reflection = 0;
    let mut possible_reflection: Option<usize> = None;
    for i in (1..end).rev() {
        if simpler_map[0] == simpler_map[i] {
            // Possible reflection
            if check_palindrome(&simpler_map[0..=i]) {
                biggest_reflection = i;
                possible_reflection = Some((1 + i) / 2);
                break;
            }
        }
    }
    for i in 0..(end - 1) {
        if simpler_map[end - 1] == simpler_map[i] {
            // Possible reflection
            if check_palindrome(&simpler_map[i..end]) {
                if end - i > biggest_reflection {
                    possible_reflection = Some((i + end) / 2);
                }
                break;
            }
        }
    }
    possible_reflection
}

fn main() {
    /*
      #.##..##.\n\
      ..#.##.#.\n\
      ##......#\n\
      ##......#\n\
      ..#.##.#.\n\
      ..##..##.\n\
      #.#.##.#.\n\
      \n\
      #...##..#\n\
      #....#..#\n\
      ..##..###\n\
      #####.##.\n\
      #####.##.\n\
      ..##..###\n\
      #....#..#\n\
    let input = "\
    ";
    */

    let input = fs::read_to_string("input.txt").unwrap();

    let subinputs: Vec<&str> = input.split("\n\n").collect();

    let mut notes = 0;
    for grid in subinputs.iter() {
        let mut ud_field: Vec<Vec<char>> = Vec::new();
        let mut lr_field: Vec<Vec<char>> = Vec::new();
        for line in grid.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            ud_field.push(row);
        }
        for j in 0..ud_field[0].len() {
            let mut new_row: Vec<char> = Vec::new();
            for i in 0..ud_field.len() {
                new_row.push(ud_field[i][j]);
            }
            lr_field.push(new_row);
        }
        if let Some(idx) = reflection(&ud_field) {
            notes += 100 * idx;
        }
        if let Some(idx) = reflection(&lr_field) {
            notes += idx;
        }
    }
    println!("notes: {}", notes);
}
