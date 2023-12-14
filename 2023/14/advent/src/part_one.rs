use std::collections::HashMap;
use std::fs;

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

    let mut cube_rocks_latest: HashMap<usize, isize> = HashMap::new();
    let mut cube_rocks: HashMap<(isize, usize), usize> = HashMap::new();
    let mut platform_size = 0;
    for (i, line) in input.lines().enumerate() {
        platform_size += 1;
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    cube_rocks_latest.insert(j, i as isize);
                }
                'O' => {
                    let row = cube_rocks_latest.get(&j).unwrap_or(&(-1));
                    cube_rocks
                        .entry((*row, j))
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
                _ => {}
            }
        }
    }
    let mut weight_burden = 0;
    for (loc, count) in cube_rocks.iter() {
        for offset in 0..*count {
            weight_burden += platform_size - (offset as isize) - (loc.0 + 1);
        }
    }
    println!("{}", weight_burden);
}
