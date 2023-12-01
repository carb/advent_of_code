use regex::Regex;
use std::fs;

fn clean_string_forwards(line: &str) -> String {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut new_line: String = re.replace_all(line, "<${1}>").into();
    new_line = new_line.replace("<one>", "1");
    new_line = new_line.replace("<two>", "2");
    new_line = new_line.replace("<three>", "3");
    new_line = new_line.replace("<four>", "4");
    new_line = new_line.replace("<five>", "5");
    new_line = new_line.replace("<six>", "6");
    new_line = new_line.replace("<seven>", "7");
    new_line = new_line.replace("<eight>", "8");
    new_line = new_line.replace("<nine>", "9");
    return new_line;
}

fn clean_string_backwards(line: &str) -> String {
    let re = Regex::new(r"(enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)").unwrap();
    let mut new_line: String = re.replace_all(line, "<${1}>").into();
    new_line = new_line.replace("<eno>", "1");
    new_line = new_line.replace("<owt>", "2");
    new_line = new_line.replace("<eerht>", "3");
    new_line = new_line.replace("<ruof>", "4");
    new_line = new_line.replace("<evif>", "5");
    new_line = new_line.replace("<xis>", "6");
    new_line = new_line.replace("<neves>", "7");
    new_line = new_line.replace("<thgie>", "8");
    new_line = new_line.replace("<enin>", "9");
    return new_line;
}

fn main() {
    // let input = "\
    //   two1nine\n\
    //   eightwothree\n\
    //   abcone2threexyz\n\
    //   xtwone3four\n\
    //   4nineeightseven2\n\
    //   zoneight234\n\
    //   7pqrstsixteen\n\
    // ";

    let input = fs::read_to_string("input.txt").unwrap();

    let mut total_calibration: u32 = 0;
    for raw_line in input.lines() {
        let line = clean_string_forwards(raw_line);
        let mut calibration_value: u32 = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                let c_string = c.to_string();
                calibration_value += 10 * c_string.parse::<u32>().unwrap();
                break;
            }
        }

        let raw_line_reverse: String = raw_line.chars().rev().collect();
        let line_reverse = clean_string_backwards(&raw_line_reverse);
        for c in line_reverse.chars() {
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
