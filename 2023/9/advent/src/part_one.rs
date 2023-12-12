use std::fs;
// use std::collections::HashMap;

fn main() {
    // let input = "\
    // 0 3 6 9 12 15\n\
    // 1 3 6 10 15 21\n\
    // 10 13 16 21 30 45\n\
    // ";

    let input = fs::read_to_string("input.txt").unwrap();

    let mut total = 0;
    for line in input.lines() {
        let mut readings: Vec<i64> = line
            .split(' ')
            .map(|n| n.trim().parse::<i64>().unwrap())
            .collect();
        let mut offset = 1;
        let mut zero_count = 1;
        while zero_count != 0 {
            zero_count = 0;
            for i in 0..readings.len() - offset {
                readings[i] = readings[i + 1] - readings[i];
                zero_count |= readings[i];
            }
            offset += 1;
        }
        total += readings.iter().sum::<i64>();
    }
    println!("{}", total);
}
