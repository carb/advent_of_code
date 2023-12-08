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

    let mut focus = node_map.get("AAA").unwrap();
    let mut found_zzz = false;
    let mut count = 0;
    while !found_zzz {
        for direction in path.chars() {
            count += 1;
            if direction == 'R' {
                focus = node_map.get(&focus.borrow().right.clone().expect("right node missing").borrow().id).unwrap();
            } else if direction == 'L' {
                focus = node_map.get(&focus.borrow().left.clone().expect("left node missing").borrow().id).unwrap();
            } else {
                continue
            }
            if focus.borrow().id == "ZZZ" {
                found_zzz = true
            }
        }
    }
    println!("{}", count);
}
