use itertools::Itertools;
use std::fs;

fn layout_check(layout: &Vec<char>, list: &Vec<usize>) -> bool {
    let mut list_ptr = 0;
    let mut current_count = 0;
    for c in layout {
        if *c == '#' {
            current_count += 1;
        } else {
            if current_count != 0 {
                if current_count == list[list_ptr] {
                    list_ptr += 1;
                } else {
                    return false;
                }
            }
            current_count = 0;
        }
    }
    return list_ptr == list.len() - 1 || list_ptr == list.len();
}

fn main() {
    // let input = "\
    //   ???.### 1,1,3\n\
    //   .??..??...?##. 1,1,3\n\
    //   ?#?#?#?#?#?#?#? 1,3,1,6\n\
    //   ????.#...#... 4,1,1\n\
    //   ????.######..#####. 1,6,5\n\
    //   ?###???????? 3,2,1\n\
    // ";

    let input = fs::read_to_string("input.txt").unwrap();

    let mut total = 0;
    for line in input.lines() {
        let known_functional = line.chars().filter(|&c| c == '#').count();
        let mut parts = line.split(' ');
        let mut hot_spring_layout: Vec<char> = parts
            .nth(0)
            .expect("")
            .chars()
            .map(|x| {
                if x == '?' {
                    return '.';
                }
                x
            })
            .collect();
        let hot_spring_list: Vec<usize> = parts
            .nth(0)
            .expect("")
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect();
        let total_functional = hot_spring_list.iter().fold(0, |acc, x| acc + x);

        let mut blanks = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c == '?' {
                blanks.push(i);
            }
        }

        let combinations = blanks
            .into_iter()
            .combinations(total_functional - known_functional);
        for combination in combinations {
            for blank_i in combination.iter() {
                hot_spring_layout[*blank_i] = '#'
            }
            if layout_check(&hot_spring_layout, &hot_spring_list) {
                total += 1;
            }
            for blank_i in combination.iter() {
                hot_spring_layout[*blank_i] = '.'
            }
        }
    }
    println!("Total: {}", total);
}
