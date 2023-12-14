use std::collections::HashMap;
use std::fs;

fn rotate_matrix(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut rotated = vec![vec![' '; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated[j][rows - 1 - i] = matrix[i][j];
        }
    }

    rotated
}

fn tilt_north(mut platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for j in 0..(platform[0].len()) {
        let mut hash_ptr = -1;
        for i in 0..(platform.len()) {
            match platform[i][j] {
                '#' => hash_ptr = i as isize,
                'O' => {
                    platform[i][j] = '.';
                    platform[(hash_ptr + 1) as usize][j] = 'O';
                    hash_ptr += 1;
                }
                _ => {}
            }
        }
    }
    return platform;
}

fn run_cycle(mut platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    platform = tilt_north(platform);
    platform = tilt_north(rotate_matrix(platform));
    platform = tilt_north(rotate_matrix(platform));
    platform = tilt_north(rotate_matrix(platform));
    rotate_matrix(platform)
}

fn main() {
    /*
    let input = "\
      O....#....\n\
      O.OO#....#\n\
      .....##...\n\
      OO.#O....O\n\
      .O.....O#.\n\
      O.#..O.#.#\n\
      ..O..#O..O\n\
      .......O..\n\
      #....###..\n\
      #OO..#....\n\
    ";
        */
    let input = fs::read_to_string("input.txt").unwrap();

    let mut platform = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        platform.push(row);
    }

    let mut cache = HashMap::new();
    let mut locations = HashMap::new();
    let mut start = 0;
    let mut period = 0;
    let iterations = 1000000000;
    for i in 0..iterations {
        if let Some(_idx) = cache.get(&platform) {
            if start == 0 {
                start = i;
                cache = HashMap::new();
            } else {
                period = i - start - 1;
                break;
            }
        } else {
            cache.insert(platform.clone(), i);
            locations.insert(i, platform.clone());
        }
        platform = run_cycle(platform);
    }

    let final_position = locations
        .get(&(start + ((iterations - start) % period)))
        .expect(&format!(
            "No cache value for i: {}",
            start + ((iterations - start) % period)
        ));

    let mut weight_burden = 0;
    let mut platform_size = final_position.len();
    for (i, row) in final_position.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c {
                'O' => {
                    weight_burden += platform_size - i;
                }
                _ => {}
            }
        }
    }
    println!("{}", weight_burden);
}
