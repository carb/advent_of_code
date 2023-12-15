use std::fs;

fn main() {
    /*
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    */
    let input = fs::read_to_string("input.txt").unwrap();

    let mut periscope: Vec<Vec<(Vec<char>, usize)>> = vec![vec![]; 256];
    for step in input.split(',') {
      let mut lens_box = 0;
      let mut fetch_focal = false;
      let mut label = Vec::new();
      let mut focal_length = 0;
      for c in step.trim().chars() {
          match c {
              '=' => {
                  fetch_focal = true;
              },
              '-' => {},
              _ => {
                if !fetch_focal {
                  lens_box += c as usize;
                  lens_box *= 17;
                  lens_box &= 0b11111111;
                  label.push(c);
                  continue;
                }
                focal_length = c.to_digit(10).unwrap() as usize;
              }
          }
      }
      if !fetch_focal {
          for i in 0..periscope[lens_box].len() {
              if periscope[lens_box][i].0 == label {
                  periscope[lens_box].remove(i);
                  break;
              }
          }
      } else {
          let mut replaced = false;
          for i in 0..periscope[lens_box].len() {
              if periscope[lens_box][i].0 == label {
                  periscope[lens_box][i] = (label.clone(), focal_length);
                  replaced = true;
                  break;
              }
          }
          if !replaced {
              periscope[lens_box].push((label, focal_length));
          }
      }
    }
    let mut focusing_power = 0;
    for i in 0..periscope.len() {
       for j in 0..periscope[i].len() {
         focusing_power += (i+1) * (j+1) * periscope[i][j].1;
       }
    }
    // println!("{:?}", periscope);
    println!("{}", focusing_power);
}
