use std::collections::HashSet;
use std::fs;

fn main() {
    /*
        let input = "\
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....
    ";
    */

    let input = fs::read_to_string("input.txt").unwrap();

    let mut field: Vec<Vec<char>> = Vec::new();
    let mut only_space_rows: HashSet<usize> = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        let mut only_space = true;
        for c in line.chars() {
            if c != '.' {
                only_space = false;
            }
            row.push(c);
        }
        if only_space {
            only_space_rows.insert(i);
        }
        field.push(row);
    }
    let mut only_space_cols: HashSet<usize> = HashSet::new();
    for i in 0..field.len() {
        let mut only_space = true;
        for j in 0..field[i].len() {
            if field[j][i] != '.' {
                only_space = false;
            }
        }
        if only_space {
            only_space_cols.insert(i);
        }
    }

    let mut corrected_field: Vec<Vec<char>> = Vec::new();
    for i in 0..field.len() {
        let mut row: Vec<char> = Vec::new();
        for j in 0..field[i].len() {
            row.push(field[i][j].clone());
            if only_space_cols.contains(&j) {
                row.push('.');
            }
        }
        let row_len = row.len();
        corrected_field.push(row);
        if only_space_rows.contains(&i) {
            let mut second_row: Vec<char> = Vec::new();
            for _j in 0..row_len {
                second_row.push('.');
            }
            corrected_field.push(second_row);
        }
    }

    let mut galaxies: Vec<(i32, i32)> = Vec::new();
    for (i, row) in corrected_field.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    let mut total_distance = 0;
    for a in 0..galaxies.len() {
        for b in a..galaxies.len() {
            total_distance +=
                (galaxies[a].0 - galaxies[b].0).abs() + (galaxies[a].1 - galaxies[b].1).abs()
        }
    }

    println!("{:?}", total_distance);
}
