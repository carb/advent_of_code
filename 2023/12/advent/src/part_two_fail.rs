use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SpringCacheKey {
    spring_map: Vec<char>,
    spring_list: Vec<usize>,
    left_edge: bool,
    right_edge: bool,
}

type SpringCache = HashMap<SpringCacheKey, usize>;

fn dp_layout_check(
    cache: &mut SpringCache,
    hot_spring_layout: &Vec<char>,
    hot_spring_list: &Vec<usize>,
    depth: usize,
    left_edge: bool,
    right_edge: bool,
) -> usize {
    let cache_key = SpringCacheKey {
        spring_map: hot_spring_layout.clone(),
        spring_list: hot_spring_list.clone(),
        left_edge: left_edge,
        right_edge: right_edge,
    };

    let target_layout = hot_spring_layout.iter().collect::<String>();

    if let Some(cached_total) = cache.get(&cache_key) {
        return *cached_total;
    }

    let mut total = 0;

    if hot_spring_list.len() == 1 {
        let functional_needed = hot_spring_list[0];
        let known_functional = hot_spring_layout.iter().filter(|&c| *c == '#').count();

        let mut allowed_placements = Vec::new();
        for (i, c) in hot_spring_layout.iter().enumerate() {
            if *c == '?' {
                allowed_placements.push(i);
            }
        }
        if allowed_placements.len() > 0 && allowed_placements[0] == 0 && !left_edge {
            allowed_placements.remove(0);
        }
        if let Some(last) = allowed_placements.last() {
            if *last == hot_spring_layout.len() - 1 && !right_edge {
               allowed_placements.pop();
            }
        }
        if known_functional == functional_needed {
            cache.insert(cache_key, 1);
            return 1;
        }
        if allowed_placements.len() + known_functional < functional_needed {
            cache.insert(cache_key, 0);
            return 0
        }
        if known_functional > functional_needed {
            cache.insert(cache_key, 0);
            return 0;
        }

        let combinations = allowed_placements
            .into_iter()
            .combinations(functional_needed - known_functional);
        for combination in combinations {
            let layout_results =
                layout_check(&combination, hot_spring_layout.clone(), &hot_spring_list);
            if layout_results.0 {
                total += 1;
            }
        }
    }
    // For each number in the list, do a split.
    for i in 1..hot_spring_list.len() {
        // Split with each possible char split on the map
        let mut left_list = hot_spring_list.clone();
        let right_list = left_list.split_off(i);
        for j in 1..hot_spring_layout.len() {
            let mut left_map = hot_spring_layout.clone();
            let right_map = left_map.split_off(j);

            let left_total = dp_layout_check(cache, &left_map, &left_list, depth+1, left_edge, false);
            let right_total = dp_layout_check(cache, &right_map, &right_list, depth+1, false, right_edge);

            // if depth == 1 && target_layout == "??.###????.###????.###????.###????.###" {
            // if depth == 2 && target_layout == ".###????.###????.###????.###????.###" {
            // if depth == 3 && target_layout == "????.###????.###????.###????.###" {
            // if target_layout == "????" && *hot_spring_list == vec![1, 1] {
            // if depth == 4 && target_layout == "????." && hot_spring_list == &vec![1, 1] {
            // if depth == 4 && target_layout == "????.###????.###????.###????.###" {
                println!("");
                println!( "{} {} {} {:?}", left_edge, hot_spring_layout.iter().collect::<String>(), right_edge, hot_spring_list);
                println!( "LEFT:\t{} {:?}", left_map.iter().collect::<String>(), left_list);
                println!( "RIGHT:\t{} {:?}", right_map.iter().collect::<String>(), right_list);
                println!("{:?}", left_total);
                println!("{:?}", right_total);
            // }
            total += left_total * right_total;
        }
    }

    /*
    println!("");
    println!("{}", hot_spring_layout.iter().collect::<String>());
    println!("{:?}", hot_spring_list);
    println!("{:?}", total);
    */
    cache.insert(cache_key, total);
    return total;
}

// use std::fs;
fn layout_check(blanks: &Vec<usize>, mut layout: Vec<char>, list: &Vec<usize>) -> (bool, usize) {
    for blank_i in blanks.iter() {
        layout[*blank_i] = '#'
    }
    // println!("{}", layout.iter().collect::<String>());

    let mut list_ptr = 0;
    let mut current_count = 0;
    for (c_i, c) in layout.iter().enumerate() {
        if *c == '#' {
            current_count += 1;
            if current_count > list[list_ptr] {
                return (false, c_i);
            }
        } else {
            if current_count != 0 && current_count == list[list_ptr] {
                list_ptr += 1;
            }
            current_count = 0;
        }
    }
    if list_ptr >= list.len() {
        return (true, layout.len());
    }
    return (current_count == list[list_ptr], layout.len());
}

fn main() {
    /*
     ???.### 1,1,3\n\
     .??..??...?##. 1,1,3\n\
     ?#?#?#?#?#?#?#? 1,3,1,6\n\
     ????.#...#... 4,1,1\n\
     ????.######..#####. 1,6,5\n\
     ?###???????? 3,2,1\n\
    */
    let input = "\
       ???.### 1,1,3\n\
    ";

    // let input = fs::read_to_string("input.txt").unwrap();
    // for a continuous string of "#?"
    /*
    let mut processed_input = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(' ');
        let hot_spring_layout = parts.nth(0).expect("").to_string();
        let hot_spring_list = parts.nth(0).expect("").to_string();
        let new_layout = std::iter::repeat(hot_spring_layout)
            .take(5)
            .collect::<Vec<String>>()
            .join("?");
        let new_list = std::iter::repeat(hot_spring_list)
            .take(5)
            .collect::<Vec<String>>()
            .join(",");
        processed_input.push(format!("{} {}", new_layout, new_list));
    }
    */

    let mut cache: SpringCache = HashMap::new();
    let mut total = 0;
    for line in input.lines() {
    // for line in processed_input {
        let mut parts = line.split(' ');
        let hot_spring_layout: Vec<char> = parts.nth(0).expect("").chars().collect();
        let hot_spring_list: Vec<usize> = parts
            .nth(0)
            .expect("")
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect();
        let partial_total = dp_layout_check(&mut cache, &hot_spring_layout, &hot_spring_list, 0, true, true);
        println!("");
        println!("{:?}", line);
        println!("Partial Total: {}", partial_total);
        total += partial_total;
    }
    println!("Total: {}", total);

    // for line in processed_input.par_iter() {
    // processed_input.par_iter().for_each(|line| {
    //     let known_functional = line.chars().filter(|&c| c == '#').count();
    //     let mut parts = line.split(' ');
    //     let hot_spring_layout: Vec<char> = parts
    //         .nth(0)
    //         .expect("")
    //         .chars()
    //         .map(|x| {
    //             if x == '?' {
    //                 return '.';
    //             }
    //             x
    //         })
    //         .collect();
    //     let hot_spring_list: Vec<usize> = parts
    //         .nth(0)
    //         .expect("")
    //         .split(',')
    //         .map(|x| x.trim().parse::<usize>().unwrap())
    //         .collect();
    //     let total_functional = hot_spring_list.iter().fold(0, |acc, x| acc + x);

    //     let mut blanks = Vec::new();
    //     for (c_i, c) in line.chars().enumerate() {
    //         if c == '?' {
    //             blanks.push(c_i);
    //         }
    //     }
    //     // println!("{}", hot_spring_layout.iter().collect::<String>());
    //     // println!("{}", total_functional - known_functional);
    //     // println!("{:?}", blanks);
    //     // let combinations = blanks
    //     //     .iter()
    //     //     .combinations(total_functional - known_functional);

    //     let mut partial_total = 0;
    //     // println!(
    //     //     "{:?}",
    //     //     n_choose_k(blanks.len(), total_functional - known_functional)
    //     // );

    //     let n = blanks.len();
    //     let k = total_functional - known_functional;
    //     let mut mask = (0..k).collect::<Vec<usize>>();
    //     let mut loop_count = 0;
    //     loop {
    //         loop_count += 1;
    //         let combination = mask.clone();

    //         // println!(
    //         //     "CHECKING {} IS {:?}",
    //         //     hot_spring_layout.iter().collect::<String>(),
    //         //     hot_spring_list,
    //         // );
    //         let mut blank_selection = Vec::new();
    //         for blank_i in combination.iter() {
    //             blank_selection.push(blanks[*blank_i]);
    //         }
    //         let layout_results = layout_check(&blank_selection, hot_spring_layout.clone(), &hot_spring_list);
    //         if layout_results.0 {
    //             // println!("FOUND ONE");
    //             // println!("\tMASK BEFORE:\t{:?}", mask);
    //             let mut layout_debug = hot_spring_layout.clone();
    //             for blank_i in blank_selection.iter() {
    //                 layout_debug[*blank_i] = '#'
    //             }
    //             // println!("{}", layout_debug.iter().collect::<String>());
    //             partial_total += 1;
    //         // } else {
    //         //     println!(
    //         //         "{} IS NOT {:?}",
    //         //         hot_spring_layout.iter().collect::<String>(),
    //         //         hot_spring_list,
    //         //     );
    //            /*
    //         } else {
    //            let mut last_blank = 0;
    //            for (blank_i, blank) in blank_selection.iter().enumerate() {
    //                // hot_spring_layout[**blank_i] = '#'
    //                last_blank = blank_i;
    //                if blank > &layout_results.1 {
    //                    break;
    //                }
    //            }
    //            /*
    //            if layout_results.1 >= 32 {
    //              println!("early exit at: {}", layout_results.1);
    //              println!("blank_selecti: {:?}", blank_selection);
    //              println!("last_blank: {:?}", last_blank);
    //              println!("end: {:?}", n - k + last_blank - 1);
    //              println!("\tMASK BEFORE:\t{:?}", mask);
    //            }
    //            */
    //            if last_blank+1 < mask.len() {
    //              mask[last_blank+1] = n - k + last_blank + 1;
    //              for j in last_blank+2..k {
    //                  mask[j] = mask[j - 1] + 1;
    //              }
    //            }
    //            if layout_results.1 >= 32 {
    //            println!("\tMASK AFTER:\t{:?}", mask);
    //            }
    //            */
    //         }
    //         // println!("MASK BEFORE:\t{:?}", mask);
    //         let mut i = k;
    //         while i > 0 && mask[i - 1] == n - k + i - 1 {
    //             // println!("mask[{} - 1]: {} == {}", i, mask[i-1], n - k + i - 1);
    //             i -= 1;
    //         }
    //         // println!("n: {}", n);
    //         // println!("i: {} -> {}", k, i);
    //         // println!("end_condition: {}", n - k + i - 1);

    //         if i == 0 {
    //             break; // All combinations generated
    //         }

    //         mask[i - 1] += 1;
    //         for j in i..k {
    //             mask[j] = mask[j - 1] + 1;
    //         }
    //         // println!("MASK AFTER:\t{:?}", mask);
    //     }
    //     println!("");
    //     println!("{:?}", line);
    //     println!("Partial Total: {}", partial_total);
    //     println!("Loop Count: {}", loop_count);
    //     println!("");
    //     // total += partial_total;
    // });
    // println!("Total: {}", total);
}
