use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn main() {
    // let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n\
    // ";

    let input = fs::read_to_string("input.txt").unwrap();

    let mut total_cards = 0;
    let mut card_dups: VecDeque<i32> = VecDeque::new();

    for line in input.lines() {
        let mut parts = line.split(&[':', '|'][..]);

        let mut dup_count = 1;
        if let Some(v) = card_dups.pop_front() {
            dup_count += v;
        }

        let _card_id = parts
            .nth(0)
            .expect("")
            .split(' ')
            .filter(|&x| !x.is_empty())
            .nth(1)
            .expect("")
            .parse::<i32>()
            .unwrap();

        let mut winners: HashSet<String> = HashSet::new();
        for winning_number in parts
            .nth(0)
            .expect("")
            .split(' ')
            .filter(|&x| !x.is_empty())
        {
            winners.insert(winning_number.to_string());
        }

        let mut match_count = 0;
        for card_number in parts
            .nth(0)
            .expect("")
            .split(' ')
            .filter(|&x| !x.is_empty())
        {
            if winners.contains(card_number) {
                match_count += 1;
            }
        }

        for x in 0..match_count {
            if card_dups.len() > x {
                card_dups[x] += dup_count;
            } else {
                card_dups.push_back(dup_count);
            }
        }
        total_cards += dup_count;
    }

    println!("{}", total_cards);
}
