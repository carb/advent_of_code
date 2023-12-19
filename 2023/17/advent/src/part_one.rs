use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    Omni,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LavaPath {
    heat_loss: usize,

    prev_pos: [usize; 2],
    prev_dir: Direction,
    time_since_turn: usize,

    visited: HashSet<[usize; 2]>,
}

impl PartialOrd for LavaPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.heat_loss.partial_cmp(&other.heat_loss)
    }
}

impl Ord for LavaPath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat_loss.cmp(&other.heat_loss)
    }
}

fn travel(field: &Vec<Vec<usize>>, pos: &[usize; 2], dir: &Direction) -> Option<[usize; 2]> {
    match dir {
        Direction::Omni => Some(pos.clone()),
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

fn inspect(field: &Vec<Vec<usize>>, pos: &[usize; 2]) -> usize {
    field[pos[0]][pos[1]]
}

fn main() {
    /*
    let input = "\
      2413432311323\n\
      3215453535623\n\
      3255245654254\n\
      3446585845452\n\
      4546657867536\n\
      1438598798454\n\
      4457876987766\n\
      3637877979653\n\
      4654967986887\n\
      4564679986453\n\
      1224686865563\n\
      2546548887735\n\
      4322674655533\n\
    ";
    */
    let input = fs::read_to_string("input.txt").unwrap();

    let mut field: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        field.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect(),
        );
    }

    let mut best_at_location: HashMap<([usize; 2], Direction, usize), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let start_path = LavaPath {
        heat_loss: 0,
        prev_pos: [0, 0],
        prev_dir: Direction::Omni,
        time_since_turn: 0,
        visited: HashSet::new(),
    };
    heap.push(Reverse(start_path));

    let mut best_visited = HashSet::new();
    let mut min_heat_loss = usize::MAX;
    while let Some(path) = heap.pop() {
        if path.0.prev_pos[0] == field.len() - 1 && path.0.prev_pos[1] == field[0].len() - 1 {
            min_heat_loss = path.0.heat_loss;
            best_visited = path.0.visited.clone();
            break;
        }
        for dir in &[
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ] {
            if let Some(new_pos) = travel(&field, &path.0.prev_pos, &dir) {
                if path.0.visited.contains(&new_pos) {
                    continue;
                }

                let mut tst = 0;
                if *dir == path.0.prev_dir {
                    tst = path.0.time_since_turn + 1;
                }
                if tst >= 3 {
                    continue;
                }

                let new_heat_loss = path.0.heat_loss + inspect(&field, &new_pos);
                if new_heat_loss
                    >= *best_at_location
                        .get(&(new_pos, dir.clone(), tst))
                        .unwrap_or(&usize::MAX)
                {
                    continue;
                }
                best_at_location.insert((new_pos.clone(), dir.clone(), tst), new_heat_loss);

                let mut new_visited = path.0.visited.clone();
                new_visited.insert(new_pos.clone());

                let new_path = LavaPath {
                    heat_loss: new_heat_loss,
                    prev_pos: new_pos,
                    prev_dir: dir.clone(),
                    time_since_turn: tst,
                    visited: new_visited,
                };
                heap.push(Reverse(new_path));
            }
        }
    }

    for (x, line) in field.iter().enumerate() {
        let mut row = Vec::new();
        for (y, c) in line.iter().enumerate() {
            if best_visited.contains(&[x, y]) {
                row.push(format!("({})", (b'0' + *c as u8) as char));
            } else {
                row.push(format!(" {} ", (b'0' + *c as u8) as char));
            }
        }
        println!("{}", row.join("\t"));
    }

    println!("{:?}", min_heat_loss);
}
