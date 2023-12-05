use std::fs;

fn compute_map_range(input: String) -> Vec<(i64, i64, i64)> {
    let mut map_range: Vec<(i64, i64, i64)> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let destination = parts[0].parse::<i64>().unwrap();
        let source = parts[1].parse::<i64>().unwrap();
        let step = parts[2].parse::<i64>().unwrap();
        map_range.push((source, source + step, destination));
    }
    map_range.sort();
    return map_range;
}

fn map_range_lookup(map_range: &Vec<(i64, i64, i64)>, value: i64) -> i64 {
    match map_range.binary_search_by(|&(a, b, _)| {
        if value >= a && value < b {
            std::cmp::Ordering::Equal
        } else if value < a {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }) {
        Ok(index) => {
            return (value - map_range[index].0) + map_range[index].2;
        }
        Err(_) => {
            return value;
        }
    }
    /*
    for block in map_range.iter() {
        if value >= block.0 && value < block.1 {
            return (value - block.0) + block.2;
        }
    }
    return value;
    */
}

fn main() {
    /*
        let seeds = "79 14 55 13";

        let seed_to_soil = "\
    50 98 2\n\
    52 50 48\n\
    ";

        let soil_to_fertilizer = "\
    0 15 37\n\
    37 52 2\n\
    39 0 15\n\
    ";

        let fertilizer_to_water = "\
    49 53 8\n\
    0 11 42\n\
    42 0 7\n\
    57 7 4\n\
    ";

        let water_to_light = "\
    88 18 7\n\
    18 25 70\n\
    ";

        let light_to_temperature = "\
    45 77 23\n\
    81 45 19\n\
    68 64 13\n\
    ";

        let temperature_to_humidity = "\
    0 69 1\n\
    1 0 69\n\
    ";

        let humidity_to_location = "\
    60 56 37\n\
    56 93 4\n\
    ";
        */

    let seeds_raw = fs::read_to_string("seeds.txt").unwrap();
    let seed_to_soil = fs::read_to_string("seed-to-soil.txt").unwrap();
    let soil_to_fertilizer = fs::read_to_string("soil-to-fertilizer.txt").unwrap();
    let fertilizer_to_water = fs::read_to_string("fertilizer-to-water.txt").unwrap();
    let water_to_light = fs::read_to_string("water-to-light.txt").unwrap();
    let light_to_temperature = fs::read_to_string("light-to-temperature.txt").unwrap();
    let temperature_to_humidity = fs::read_to_string("temperature-to-humidity.txt").unwrap();
    let humidity_to_location = fs::read_to_string("humidity-to-location.txt").unwrap();

    let seed_to_soil_map = compute_map_range(seed_to_soil);
    let soil_to_fertilizer_map = compute_map_range(soil_to_fertilizer);
    let fertilizer_to_water_map = compute_map_range(fertilizer_to_water);
    let water_to_light_map = compute_map_range(water_to_light);
    let light_to_temperature_map = compute_map_range(light_to_temperature);
    let temperature_to_humidity_map = compute_map_range(temperature_to_humidity);
    let humidity_to_location_map = compute_map_range(humidity_to_location);

    let mut min_location: i64 = 999999999999;
    let seeds: Vec<i64> = seeds_raw
        .split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    for i in 0..(seeds.len() / 2) {
        for seed in seeds[2 * i]..(seeds[2 * i] + seeds[2 * i + 1]) {
            let soil = map_range_lookup(&seed_to_soil_map, seed);
            let fertilizer = map_range_lookup(&soil_to_fertilizer_map, soil);
            let water = map_range_lookup(&fertilizer_to_water_map, fertilizer);
            let light = map_range_lookup(&water_to_light_map, water);
            let temperature = map_range_lookup(&light_to_temperature_map, light);
            let humidity = map_range_lookup(&temperature_to_humidity_map, temperature);
            let location = map_range_lookup(&humidity_to_location_map, humidity);
            if location < min_location {
                min_location = location;
            }
        }
    }
    println!("{}", min_location);
}
