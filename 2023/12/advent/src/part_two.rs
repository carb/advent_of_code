use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SpringCacheKey {
    spring_map: Vec<char>,
    spring_list: Vec<usize>,
}

type SpringCache = HashMap<SpringCacheKey, usize>;

fn dp_layout_check(
    cache: &mut SpringCache,
    hot_spring_layout: &Vec<char>,
    hot_spring_list: &Vec<usize>,
    depth: usize,
) -> usize {
    let cache_key = SpringCacheKey {
        spring_map: hot_spring_layout.clone(),
        spring_list: hot_spring_list.clone(),
    };

    if let Some(cached_total) = cache.get(&cache_key) {
        return *cached_total;
    }

    if hot_spring_layout.len() == 0 {
        if hot_spring_list.len() == 0 {
            cache.insert(cache_key, 1);
            return 1;
        }
        cache.insert(cache_key, 0);
        return 0;
    }

    if hot_spring_list.len() == 0 {
        if hot_spring_layout.iter().filter(|&c| *c == '#').count() > 0 {
            cache.insert(cache_key, 0);
            return 0;
        }
        cache.insert(cache_key, 1);
        return 1;
    }

    let functional_needed = hot_spring_list.iter().fold(0, |acc, x| acc + x);
    // TODO Why this?
    if hot_spring_layout.len() < functional_needed + hot_spring_list.len() - 1 {
        cache.insert(cache_key, 0);
        return 0;
    }

    if hot_spring_layout[0] == '.' {
        let total = dp_layout_check(
            cache,
            &hot_spring_layout[1..].to_vec(),
            &hot_spring_list,
            depth + 1,
        );
        cache.insert(cache_key, total);
        return total;
    }

    if hot_spring_layout[0] == '#' {
        let num = hot_spring_list[0];
        for i in 0..num {
            if hot_spring_layout[i] == '.' {
                cache.insert(cache_key, 0);
                return 0;
            }
        }

        if hot_spring_layout.len() < num {
            cache.insert(cache_key, 0);
            return 0;
        }
        if num < hot_spring_layout.len() && hot_spring_layout[num] == '#' {
            cache.insert(cache_key, 0);
            return 0;
        }

        if num + 1 < hot_spring_layout.len() {
            let total = dp_layout_check(
                cache,
                &hot_spring_layout[num + 1..].to_vec(),
                &hot_spring_list[1..].to_vec(),
                depth + 1,
            );
            cache.insert(cache_key, total);
            return total;
        } else {
            let total = dp_layout_check(cache, &vec![], &hot_spring_list[1..].to_vec(), depth + 1);
            cache.insert(cache_key, total);
            return total;
        }
    }

    let mut spring_layout = hot_spring_layout.clone();
    spring_layout[0] = '#';
    let mut gap_layout = hot_spring_layout.clone();
    gap_layout[0] = '.';

    let spring_total = dp_layout_check(cache, &spring_layout, &hot_spring_list, depth + 1);
    let gap_total = dp_layout_check(cache, &gap_layout, &hot_spring_list, depth + 1);

    let total = spring_total + gap_total;
    cache.insert(cache_key, total);
    return total;
}

fn main() {
    /*
    let input = "\
       ???.### 1,1,3\n\
     .??..??...?##. 1,1,3\n\
     ?#?#?#?#?#?#?#? 1,3,1,6\n\
     ????.#...#... 4,1,1\n\
     ????.######..#####. 1,6,5\n\
     ?###???????? 3,2,1\n\
    ";
    */

    let input = fs::read_to_string("input.txt").unwrap();
    // for a continuous string of "#?"
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

    let mut cache: SpringCache = HashMap::new();
    let mut total = 0;
    // for line in input.lines() {
    for line in processed_input {
        let mut parts = line.split(' ');
        let hot_spring_layout: Vec<char> = parts.nth(0).expect("").chars().collect();
        let hot_spring_list: Vec<usize> = parts
            .nth(0)
            .expect("")
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect();
        let partial_total = dp_layout_check(&mut cache, &hot_spring_layout, &hot_spring_list, 0);
        println!("");
        println!("{:?}", line);
        println!("Partial Total: {}", partial_total);
        total += partial_total;
    }
    println!("Total: {}", total);
}
