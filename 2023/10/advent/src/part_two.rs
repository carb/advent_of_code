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
    ......................\n\
    .FF7FSF7F7F7F7F7F---7.\n\
    .L|LJ||||||||||||F--J.\n\
    .FL-7LJLJ||||||LJL-77.\n\
    .F--JF--7||LJLJ7F7FJ-.\n\
    .L---JF-JLJ.||-FJLJJ7.\n\
    .|F|F-JF---7F7-L7L|7|.\n\
    .|FFJF7L7F-JF7|JL---7.\n\
    .7-L-JL7||F7|L7F-7F7|.\n\
    .L.L7LFJ|||||FJL7||LJ.\n\
    .L7JLJL-JLJLJL--JLJ.L.\n\
    ......................\n\
        ";
        ..........\n\
        .S------7.\n\
        .|F----7|.\n\
        .||....||.\n\
        .||....||.\n\
        .|L-7F-J|.\n\
        .|..||..|.\n\
        .L--JL--J.\n\
        ..........\n\
        */

    let input = fs::read_to_string("input.txt").unwrap();

    let mut field: Vec<Vec<char>> = Vec::new();
    let mut mask: Vec<Vec<char>> = Vec::new();
    let mut start: [usize; 2] = Default::default();
    for (i, line) in input.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        let mut mask_row: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start[0] = i;
                start[1] = j;
                mask_row.push('S');
            } else {
                mask_row.push(' ');
            }
            row.push(c);
        }
        field.push(row);
        mask.push(mask_row);
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

    while location_dasher != location_blitzen {
        mask[location_dasher[0]][location_dasher[1]] = inspect(&field, &location_dasher);
        mask[location_blitzen[0]][location_blitzen[1]] = inspect(&field, &location_blitzen);

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
    mask[location_dasher[0]][location_dasher[1]] = inspect(&field, &location_dasher);

    /*
    println!("{}", input);
    for line in mask.iter() {
        println!("{}", line.iter().collect::<String>());
    }
    */

    let mut taller_mask: Vec<Vec<char>> = Vec::new();
    for i in 0..mask.len() {
        let mut row: Vec<char> = Vec::new();
        let mut new_row: Vec<char> = Vec::new();
        for c in mask[i].iter() {
            row.push(c.clone());
            match c {
                'S' => new_row.push('|'),
                '|' => new_row.push('|'),
                '7' => new_row.push('|'),
                'F' => new_row.push('|'),
                _ => new_row.push('/'), // '·'),
            }
        }
        taller_mask.push(row);
        taller_mask.push(new_row);
    }

    let mut zoomed_mask: Vec<Vec<char>> = Vec::new();
    for i in 0..taller_mask.len() {
        let mut row: Vec<char> = Vec::new();
        for c in taller_mask[i].iter() {
            row.push(c.clone());
            match c {
                'S' => row.push('-'),
                '-' => row.push('-'),
                'L' => row.push('-'),
                'F' => row.push('-'),
                _ => row.push('/'),
            }
        }
        zoomed_mask.push(row);
    }

    /*
    for line in &zoomed_mask {
        println!("{}", line.iter().collect::<String>());
    }
    */

    let dx = [0, 1, 0, -1];
    let dy = [-1, 0, 1, 0];
    let mut stack = Vec::new();
    stack.push((0, 0));
    while let Some((x, y)) = stack.pop() {
        zoomed_mask[x][y] = '@';
        for i in 0..4 {
            let new_x = x as isize + dx[i];
            let new_y = y as isize + dy[i];
            if new_x >= 0
                && new_y >= 0
                && new_x < zoomed_mask.len() as isize
                && new_y < zoomed_mask[0].len() as isize
            {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if zoomed_mask[new_x][new_y] == ' ' || zoomed_mask[new_x][new_y] == '/' {
                    stack.push((new_x, new_y));
                    continue;
                }
            }
        }
    }

    let mut area = 0;
    for line in &zoomed_mask {
        for c in line {
            if *c == ' ' {
                area += 1;
            }
        }
        // println!("{}", line.iter().collect::<String>());
    }

    println!("{}", area);
}
