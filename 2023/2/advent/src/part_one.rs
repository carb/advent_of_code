use std::fs;
use std::collections::HashMap;

fn main() {
    // let input = "\
    //   Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
    //   Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
    //   Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
    //   Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
    //   Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n\
    // ";

    let input = fs::read_to_string("input.txt").unwrap();

    let mut id_sum = 0;
    for line in input.lines() {
        let mut max_counts: HashMap<String, i32> = HashMap::new();
        let mut id = 0;
        for part in line.split(&[':', ';'][..]) {
          if part.starts_with('G') {
            let game_id: Vec<&str> = part.split(' ').collect();
            id = game_id[1].parse::<i32>().unwrap();
            continue
          }
          for color in part.split(',') {
              let count_parts: Vec<&str> = color.split(' ').filter(|&x| !x.is_empty()).collect();
              let count = count_parts[0].parse::<i32>().unwrap();
              let color = count_parts[1].to_string();
              max_counts.entry(color).and_modify(|e| *e = i32::max(*e, count)).or_insert(count);
          }
        }
        if max_counts.get("red").unwrap_or(&0) <= &12 &&
           max_counts.get("green").unwrap_or(&0) <= &13 &&
           max_counts.get("blue").unwrap_or(&0) <= &14 {
           id_sum += id;
        }
    }
    println!("ID Sum: {}", id_sum);
}
