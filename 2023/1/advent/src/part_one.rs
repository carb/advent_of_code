use std::fs;

fn main() {
    // let test_input = "\
    //   1abc2\n\
    //   pqr3stu8vwx\n\
    //   a1b2c3d4e5f\n\
    //   treb7uchet\n\
    // ";

    let input = fs::read_to_string("input.txt").unwrap();

    let mut total_calibration: u32 = 0;
    for line in input.lines() {
        let mut calibration_value: u32 = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                let c_string = c.to_string();
                calibration_value += 10 * c_string.parse::<u32>().unwrap();
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                let c_string = c.to_string();
                calibration_value += c_string.parse::<u32>().unwrap();
                break;
            }
        }
        total_calibration += calibration_value;
    }
    println!("{}", total_calibration);
}
