use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

lazy_static! {
    static ref REDIRECT: HashMap<(char, Direction), Direction> = {
        let mut map = HashMap::new();
        map.insert(('|', Direction::Down), Direction::Down);
        map.insert(('|', Direction::Up), Direction::Up);
        map.insert(('-', Direction::Right), Direction::Right);
        map.insert(('-', Direction::Left), Direction::Left);
        map.insert(('L', Direction::Down), Direction::Right);
        map.insert(('L', Direction::Left), Direction::Up);
        map.insert(('J', Direction::Down), Direction::Left);
        map.insert(('J', Direction::Right), Direction::Up);
        map.insert(('7', Direction::Right), Direction::Down);
        map.insert(('7', Direction::Up), Direction::Left);
        map.insert(('F', Direction::Left), Direction::Down);
        map.insert(('F', Direction::Up), Direction::Right);
        map
    };
}

fn travel(pos: &[usize; 2], dir: &Direction) -> [usize; 2] {
    match dir {
        Direction::Up => [pos[0] - 1, pos[1]],
        Direction::Right => [pos[0], pos[1] + 1],
        Direction::Down => [pos[0] + 1, pos[1]],
        Direction::Left => [pos[0], pos[1] - 1],
    }
}

fn inspect(field: &Vec<Vec<char>>, pos: &[usize; 2]) -> char {
    field[pos[0]][pos[1]]
}

fn main() {
    /*
        let input = "\
    .......\n\
    .7-F7-.\n\
    ..FJ|7.\n\
    .SJLL7.\n\
    .|F--J.\n\
    .LJ.LJ.\n\
    .......\n\
        ";
        .....\n\
        .S-7.\n\
        .|.|.\n\
        .L-J.\n\
        .....\n\
        */

    let input = fs::read_to_string("input.txt").unwrap();

    let mut field: Vec<Vec<char>> = Vec::new();
    let mut start: [usize; 2] = Default::default();
    for (i, line) in input.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start[0] = i;
                start[1] = j;
            }
            row.push(c);
        }
        field.push(row);
    }

    let mut locations: Vec<[usize; 2]> = Vec::new();
    let mut directions: Vec<Direction> = Vec::new();

    for dir in &[
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ] {
        let new_pos = travel(&start, &dir);
        if let Some(new_dir) = REDIRECT.get(&(inspect(&field, &new_pos), dir.clone())) {
            locations.push(new_pos.clone());
            directions.push(new_dir.clone());
        }
    }

    if locations.len() != 2 || directions.len() != 2 {
        eprintln!(
            "Start does not have exactly two pipes: {:?}, {:?}",
            locations, directions
        );
        std::process::exit(1);
    }

    let mut location_dasher = locations[0];
    let mut direction_dasher = directions[0].clone();
    let mut location_blitzen = locations[1];
    let mut direction_blitzen = directions[1].clone();

    let mut steps = 1;
    while location_dasher != location_blitzen {
        steps += 1;
        location_dasher = travel(&location_dasher, &direction_dasher);
        if let Some(dir) =
            REDIRECT.get(&(inspect(&field, &location_dasher), direction_dasher.clone()))
        {
            direction_dasher = dir.clone();
        } else {
            eprintln!(
                "Accidentally left the loop: {:?}, {:?}",
                location_dasher, direction_dasher
            );
            std::process::exit(1);
        }

        location_blitzen = travel(&location_blitzen, &direction_blitzen);
        if let Some(dir) = REDIRECT.get(&(
            inspect(&field, &location_blitzen),
            direction_blitzen.clone(),
        )) {
            direction_blitzen = dir.clone();
        } else {
            eprintln!(
                "Accidentally left the loop: {:?}, {:?}",
                location_blitzen, direction_blitzen
            );
            std::process::exit(1);
        }
    }
    println!("{:?}, {}", location_blitzen, steps);
}
