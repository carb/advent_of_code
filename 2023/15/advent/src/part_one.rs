use std::fs;

fn main() {
    /*
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    */
    let input = fs::read_to_string("input.txt").unwrap();

    let mut total_value = 0;
    for step in input.split(',') {
      let mut current_value = 0;
      for c in step.trim().chars() {
          current_value += c as u32;
          current_value *= 17;
          current_value &= 0b11111111;
      }
      total_value += current_value;
    }
    println!("{}", total_value);
}
