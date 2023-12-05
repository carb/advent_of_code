use std::fs;
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
    let mut part_number_sum = 0;
    for (index, raw_line) in input.lines().enumerate() {
      let line: Vec<char> = raw_line.chars().collect();
      for mat in re.find_iter(raw_line) {
          let mut part_number = false;
          if line[mat.start()-1] != '.' {
              part_number = true;
          }
          if line[mat.end()] != '.' {
              part_number = true;
          }
          part_number |= &lines[index-1][mat.start()-1..mat.end()+1].chars().any(|c| c != '.');
          part_number |= &lines[index+1][mat.start()-1..mat.end()+1].chars().any(|c| c != '.');
          if part_number {
              part_number_sum += mat.as_str().parse::<i32>().unwrap();
          }
      }
    }
    println!("{}", part_number_sum);
}
