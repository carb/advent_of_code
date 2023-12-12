use std::collections::HashSet;
use std::fs;

const WARP_FACTOR: usize = 1000000;

fn main() {
    /*
    let input = "\
    ...#......\n\
    .......#..\n\
    #.........\n\
    ..........\n\
    ......#...\n\
    .#........\n\
    .........#\n\
    ..........\n\
    .......#..\n\
    #...#.....\n\
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

    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (i, row) in field.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push((i, j));
            }
        }
    }

    let mut total_distance = 0;
    for a in 0..galaxies.len() {
        for b in a + 1..galaxies.len() {
            let mut part_distance = 0;

            let x_min = galaxies[a].0.min(galaxies[b].0);
            let x_max = galaxies[a].0.max(galaxies[b].0);
            for x in x_min..x_max {
                if only_space_rows.contains(&x) {
                    part_distance += WARP_FACTOR;
                } else {
                    part_distance += 1
                }
            }
            let y_min = galaxies[a].1.min(galaxies[b].1);
            let y_max = galaxies[a].1.max(galaxies[b].1);
            for y in y_min..y_max {
                if only_space_cols.contains(&y) {
                    part_distance += WARP_FACTOR
                } else {
                    part_distance += 1
                }
            }
            total_distance += part_distance;
        }
    }
    println!("{:?}", total_distance);
}

/*
let mut warp_field: Vec<Vec<char>> = Vec::new();
for i in 0..field.len() {
    let mut row: Vec<char> = Vec::new();
    if only_space_rows.contains(&i) {
        for j in 0..field[i].len() {
            if only_space_cols.contains(&j) {
               row.push('╬');
            } else {
               row.push('═');
            }
        }
        warp_field.push(row);
        continue
    }

    for j in 0..field[i].len() {
        if only_space_cols.contains(&j) {
            row.push('║');
        } else {
            row.push(field[i][j].clone());
        }
    }
    warp_field.push(row);
}

for line in &field {
    println!("{}", line.iter().collect::<String>());
}

println!("");
for line in &warp_field {
    println!("{}", line.iter().collect::<String>());
}
*/
