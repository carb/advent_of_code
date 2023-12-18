use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

lazy_static! {
    static ref REDIRECT: HashMap<(char, Direction), Vec<Direction>> = {
        let mut map = HashMap::new();
        map.insert(('\\', Direction::Down), vec![Direction::Right]);
        map.insert(('\\', Direction::Up), vec![Direction::Left]);
        map.insert(('\\', Direction::Right), vec![Direction::Down]);
        map.insert(('\\', Direction::Left), vec![Direction::Up]);

        map.insert(('/', Direction::Down), vec![Direction::Left]);
        map.insert(('/', Direction::Up), vec![Direction::Right]);
        map.insert(('/', Direction::Right), vec![Direction::Up]);
        map.insert(('/', Direction::Left), vec![Direction::Down]);

        map.insert(('-', Direction::Right), vec![Direction::Right]);
        map.insert(('-', Direction::Left), vec![Direction::Left]);
        map.insert(
            ('-', Direction::Down),
            vec![Direction::Left, Direction::Right],
        );
        map.insert(
            ('-', Direction::Up),
            vec![Direction::Left, Direction::Right],
        );

        map.insert(('|', Direction::Down), vec![Direction::Down]);
        map.insert(('|', Direction::Up), vec![Direction::Up]);
        map.insert(
            ('|', Direction::Right),
            vec![Direction::Up, Direction::Down],
        );
        map.insert(('|', Direction::Left), vec![Direction::Up, Direction::Down]);

        map.insert(('.', Direction::Right), vec![Direction::Right]);
        map.insert(('.', Direction::Left), vec![Direction::Left]);
        map.insert(('.', Direction::Down), vec![Direction::Down]);
        map.insert(('.', Direction::Up), vec![Direction::Up]);
        map
    };
}

fn travel(field: &Vec<Vec<char>>, pos: &[usize; 2], dir: &Direction) -> Option<[usize; 2]> {
    match dir {
        Direction::Up => {
            if pos[0] == 0 {
                return None;
            }
            return Some([pos[0] - 1, pos[1]]);
        }
        Direction::Right => {
            if pos[1] + 1 == field[pos[0]].len() {
                return None;
            }
            return Some([pos[0], pos[1] + 1]);
        }
        Direction::Down => {
            if pos[0] + 1 == field.len() {
                return None;
            }
            return Some([pos[0] + 1, pos[1]]);
        }
        Direction::Left => {
            if pos[1] == 0 {
                return None;
            }
            return Some([pos[0], pos[1] - 1]);
        }
    }
}

fn inspect(field: &Vec<Vec<char>>, pos: &[usize; 2]) -> char {
    field[pos[0]][pos[1]]
}

fn main() {
    /*
            let input = r".|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....";
            */

    let input = fs::read_to_string("input.txt").unwrap();

    let mut field: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        field.push(row);
    }

    let mut locations: Vec<([usize; 2], Direction)> = vec![([0, 0], Direction::Right)];
    let mut cache = HashSet::new();
    let mut energized = HashSet::new();
    while let Some(beam) = locations.pop() {
        if cache.contains(&(beam.0.clone(), beam.1.clone())) {
            continue;
        }
        cache.insert((beam.0.clone(), beam.1.clone()));
        energized.insert(beam.0.clone());

        if let Some(dirs) = REDIRECT.get(&(inspect(&field, &beam.0), beam.1.clone())) {
            for dir in dirs {
                if let Some(new_location) = travel(&field, &beam.0, &dir) {
                    locations.push((new_location, dir.clone()));
                }
            }
        } else {
            eprintln!("Unexpected character");
            std::process::exit(1);
        }
    }

    println!("{}", energized.len());
}
