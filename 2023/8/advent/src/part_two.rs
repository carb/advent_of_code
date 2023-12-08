use std::fs;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};


// Copied from https://rusty-ferris.pages.dev/blog/binary-tree-sum-of-values/
#[derive(Debug, Clone)]
struct GraphNode {
  id: String,
  left: Option<GraphNodeRef>,
  right: Option<GraphNodeRef>,
}

type GraphNodeRef = Rc<RefCell<GraphNode>>;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b).abs() / gcd(a, b)
    }
}

fn main() {
    /*
      RL\n\
      \n\
      AAA = (BBB, CCC)\n\
      BBB = (DDD, EEE)\n\
      CCC = (ZZZ, GGG)\n\
      DDD = (DDD, DDD)\n\
      EEE = (EEE, EEE)\n\
      GGG = (GGG, GGG)\n\
      ZZZ = (ZZZ, ZZZ)\n\
    let input = "\
      LLR\n\
      \n\
      AAA = (BBB, BBB)\n\
      BBB = (AAA, ZZZ)\n\
      ZZZ = (ZZZ, ZZZ)\n\
    ";
    let input = "\
      LR\n\
      \n\
      11A = (11B, XXX)\n\
      11B = (XXX, 11Z)\n\
      11Z = (11B, XXX)\n\
      22A = (22B, XXX)\n\
      22B = (22C, 22C)\n\
      22C = (22Z, 22Z)\n\
      22Z = (22B, 22B)\n\
      XXX = (XXX, XXX)\n\
    ";
    */


    let input = fs::read_to_string("input.txt").unwrap();

    let mut lines = input.lines();
    let path = lines.nth(0).unwrap();
    // skip empty
    let _blank = lines.nth(0).unwrap();

    let mut node_map: HashMap<String, GraphNodeRef> = HashMap::new();
    for line in lines {
        let mut parts = line.split(&['=', '(', ',', ')'][..]).filter(|&x| !x.trim().is_empty());
        let node_id = parts.nth(0).unwrap().trim().to_string();
        let left_id = parts.nth(0).unwrap().trim().to_string();
        let right_id = parts.nth(0).unwrap().trim().to_string();

        let left_node = node_map.entry(left_id.clone()).or_insert(
          Rc::new(RefCell::new(GraphNode {
            id: left_id.clone(),
            left: None,
            right: None,
          }))
        ).clone();

        let right_node = node_map.entry(right_id.clone()).or_insert(
          Rc::new(RefCell::new(GraphNode {
            id: right_id.clone(),
            left: None,
            right: None,
          }))
        ).clone();

        node_map.entry(node_id.clone()).or_insert(
          Rc::new(RefCell::new(GraphNode {
            id: node_id.clone(),
            left: None,
            right: None,
          }))
        );

        node_map.entry(node_id.clone()).and_modify(|n| {
            n.borrow_mut().left = Some(left_node);
            n.borrow_mut().right = Some(right_node);
        });
    }

    let mut start_nodes = Vec::new();
    let mut focuses = Vec::new();
    for (id, node) in node_map.iter() {
        if id.ends_with("A") {
            focuses.push(node.clone());
            start_nodes.push(id);
        }
    }

    let mut shortest_paths: HashMap<String, Vec<i64>> = HashMap::new();
    let mut found_zzz = false;
    let mut count = 0;
    while !found_zzz {
        for direction in path.chars() {
            count += 1;
            for i in 0..focuses.len() {
              let mut focus = &focuses[i];
              if direction == 'R' {
                  focus = node_map.get(&focus.borrow().right.clone().expect("right node missing").borrow().id).unwrap();
              } else if direction == 'L' {
                  focus = node_map.get(&focus.borrow().left.clone().expect("left node missing").borrow().id).unwrap();
              } else {
                  continue
              }
              if focus.borrow().id.ends_with("Z") {
                  shortest_paths.entry(start_nodes[i].to_string() + &focus.borrow().id).and_modify(|v| v.push(count)).or_insert(vec![count]);
              }
              focuses[i] = focus.clone();
            }
            if shortest_paths.len() == focuses.len() {
                found_zzz = true;
            }
        }
    }

    // NOTE: This works only because all the paths in the input are simple cycles with a constant
    // length. This wouldn't work if the **Z nodes didn't point directly back into the start of the
    // loop.
    let mut all_paths = shortest_paths.iter();
    let first = all_paths.nth(0).unwrap();
    let mut lcm_result = first.1[0];
    for (_path, distance) in all_paths  {
        lcm_result = lcm(lcm_result, distance[0]);
    }

    println!("{}", lcm_result);
}
