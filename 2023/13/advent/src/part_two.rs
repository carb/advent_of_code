use std::fs;

fn is_power_of_two_diff(a: i64, b: i64) -> bool {
    let n = a ^ b;
    // Check if n is greater than 0 and has only one bit set
    n > 0 && (n & (n - 1)) == 0
}

fn hash_row(row: &Vec<char>) -> i64 {
    let mut hash = 0;
    for (i, c) in row.iter().enumerate() {
        if *c == '#' {
            hash += 1 << i
        }
    }
    hash
}

fn check_palindrome_single_smudge(mirror: &[i64]) -> bool {
    let mut used_smudge = false;
    for i in 0..mirror.len() / 2 {
        if mirror[i] != mirror[mirror.len() - 1 - i] {
            if !used_smudge && is_power_of_two_diff(mirror[i], mirror[mirror.len() - 1 - i]) {
                used_smudge = true;
                continue;
            }
            return false;
        }
    }
    used_smudge
}

fn reflection(mountain: &Vec<Vec<char>>) -> Option<usize> {
    let mut simpler_map: Vec<i64> = Vec::new();
    for line in mountain {
        simpler_map.push(hash_row(line));
    }
    let end = simpler_map.len();
    let mut biggest_reflection = 0;
    let mut possible_reflection: Option<usize> = None;
    for i in (1..end).step_by(2).rev() {
        if check_palindrome_single_smudge(&simpler_map[0..=i]) {
            biggest_reflection = i;
            possible_reflection = Some((1 + i) / 2);
            break;
        }
    }
    let odd = simpler_map.len() % 2;
    for i in (odd..(end - 1)).step_by(2) {
        if check_palindrome_single_smudge(&simpler_map[i..end]) {
            if end - i > biggest_reflection {
                possible_reflection = Some((i + end) / 2);
            }
            break;
        }
    }
    possible_reflection
}

fn main() {
    /*
    let input = "\
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
