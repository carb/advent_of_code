use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    // let input = "\
    //   ............\n\
    //   .467..114...\n\
    //   ....*.......\n\
    //   ...35..633..\n\
    //   .......#....\n\
    //   .617*.......\n\
    //   ......+.58..\n\
    //   ...592......\n\
    //   .......755..\n\
    //   ....$.*.....\n\
    //   ..664.598...\n\
    //   ............\n\
    // ";

    let input = fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"\d+").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (index, raw_line) in input.lines().enumerate() {
      let line: Vec<char> = raw_line.chars().collect();
      for mat in re.find_iter(raw_line) {
          let impact = mat.as_str().parse::<i32>().unwrap();
          // before
          if line[mat.start()-1] == '*' {
              gears.entry((index, mat.start()-1)).or_insert_with(Vec::new).push(impact);
          }
          // after
          if line[mat.end()] == '*' {
              gears.entry((index, mat.end())).or_insert_with(Vec::new).push(impact);
          }
          // line before
          for (c_index, c) in lines[index-1][mat.start()-1..mat.end()+1].chars().enumerate() {
            if c != '*' {
                continue
            }
            gears.entry((index-1, mat.start()-1 + c_index)).or_insert_with(Vec::new).push(impact);
          }
          // line after
          for (c_index, c) in lines[index+1][mat.start()-1..mat.end()+1].chars().enumerate() {
            if c != '*' {
                continue
            }
            gears.entry((index+1, mat.start()-1 + c_index)).or_insert_with(Vec::new).push(impact);
          }
      }
    }

    let mut gear_sum = 0;
    for (_gear, impacts) in gears.iter() {
      if impacts.len() == 2 {
        gear_sum += impacts[0] * impacts[1];
      }
    }
    println!("{}", gear_sum);
}
