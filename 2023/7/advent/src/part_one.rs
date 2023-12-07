use lazy_static::lazy_static;
use ordered_float::NotNan;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref LABEL_VALUE: HashMap<char, f64> = {
        let mut map = HashMap::new();
        map.insert('2', 0.02);
        map.insert('3', 0.03);
        map.insert('4', 0.04);
        map.insert('5', 0.05);
        map.insert('6', 0.06);
        map.insert('7', 0.07);
        map.insert('8', 0.08);
        map.insert('9', 0.09);
        map.insert('T', 0.10);
        map.insert('J', 0.11);
        map.insert('Q', 0.12);
        map.insert('K', 0.13);
        map.insert('A', 0.14);
        map
    };
    static ref HAND_VALUE: HashMap<i64, f64> = {
        let mut map = HashMap::new();
        map.insert(2, 1.0);
        map.insert(3, 3.0);
        map.insert(4, 5.0);
        map.insert(5, 6.0);
        map
    };
}

fn score(hand: &str) -> f64 {
    let mut score = 0.0;
    let mut matches: HashMap<char, i64> = HashMap::new();
    for (index, card) in hand.chars().enumerate() {
        matches.entry(card).and_modify(|c| *c += 1).or_insert(1);
        score += 0.01_f64.powf(index as f64) * LABEL_VALUE.get(&card).expect("card not found");
    }
    for (_label, count) in matches.iter() {
        score += HAND_VALUE.get(&count).unwrap_or(&0.0);
    }
    return score;
}

fn main() {
    // let input = "\
    //   32T3K 765\n\
    //   T55J5 684\n\
    //   KK677 28\n\
    //   KTJJT 220\n\
    //   QQQJA 483\n\
    // ";

    let input = fs::read_to_string("input.txt").unwrap();

    let mut hands = BinaryHeap::new();

    for line in input.lines() {
        let mut parts = line.split(' ').filter(|&x| !x.is_empty());
        let hand = parts.nth(0).unwrap();
        let bid = parts.nth(0).unwrap().parse::<i64>().unwrap();

        hands.push((Reverse(NotNan::new(score(hand)).unwrap()), bid));
    }

    let mut index = 1;
    let mut winnings = 0;
    while let Some((Reverse(_hand_score), bid)) = hands.pop() {
        winnings += index * bid;
        index += 1;
    }
    println!("{}", winnings);
}
