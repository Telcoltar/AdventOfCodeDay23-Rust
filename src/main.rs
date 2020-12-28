mod test_solutions;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};
use std::rc::Rc;
use std::ops::{DerefMut};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>
}

fn get_input_data(filename: &str, max_val: i32) -> (HashMap<i32, Rc<RefCell<Node>>>, Rc<RefCell<Node>>) {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut nodes: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();
    let head = Node {value: 0, next: None };
    let rc_head = Rc::from(RefCell::new(head));
    let mut previous_node = rc_head.clone();
    let mut current_node: Node;
    let mut current_rc;
    let input_data = f.lines().nth(0).unwrap().unwrap();
    let digits: Vec<i32> = input_data.chars().map(|s|s.to_digit(10)).
        map(|s|s.unwrap() as i32).collect();
    previous_node.borrow_mut().deref_mut().value = digits[0];
    nodes.insert(digits[0], previous_node.clone());
    for digit in &digits[1..] {
        current_node = Node {value: *digit, next: None};
        current_rc = Rc::from(RefCell::new(current_node));
        previous_node.borrow_mut().next = Some(current_rc.clone());
        previous_node = current_rc;
        nodes.insert(*digit, previous_node.clone());
    }
    for i in 10..(max_val + 1) {
        current_node = Node {value: i, next: None};
        current_rc = Rc::from(RefCell::new(current_node));
        previous_node.borrow_mut().next = Some(current_rc.clone());
        previous_node = current_rc;
        nodes.insert(i, previous_node.clone());
    }
    previous_node.borrow_mut().next = Some(rc_head.clone());
    debug!("{:?}", digits);
    return (nodes, rc_head);
}

fn cycle(nodes: &HashMap<i32, Rc<RefCell<Node>>>, head: &Rc<RefCell<Node>>, max_value: i32) -> Rc<RefCell<Node>>{
    let next = head.clone().as_ref().borrow().next.clone();
    let next2 = next.clone().unwrap().as_ref().borrow().next.clone();
    let next3 = next2.clone().unwrap().as_ref().borrow().next.clone();
    head.borrow_mut().next = next3.clone().unwrap().as_ref().borrow().next.clone();
    let values = vec![next.clone().unwrap().as_ref().borrow().value,
                      next2.clone().unwrap().as_ref().borrow().value,
                      next3.clone().unwrap().as_ref().borrow().value];
    let mut new_value = head.as_ref().borrow().value - 1;
    if new_value < 1 {
        new_value = max_value;
    }
    while values.contains(&new_value) {
        new_value -= 1;
        if new_value < 1 {
            new_value = max_value;
        }
    }
    let dest = nodes.get(&new_value);
    next3.clone().unwrap().as_ref().borrow_mut().next = dest.unwrap().as_ref().borrow().next.clone();
    dest.unwrap().as_ref().borrow_mut().next = next;
    return head.borrow().next.as_ref().unwrap().clone();
}

fn solution_part_1(filename: &str, rounds: i32) -> String {
    let (nodes, head) = get_input_data(filename, 9);
    let mut next = head.clone();
    for _i in 0..rounds {
        next = cycle(&nodes, &next, 9);
    }
    let mut current = Some(nodes.get(&1).unwrap().to_owned());
    let mut result = String::new();
    for _i in 0..8 {
        current = current.unwrap().as_ref().borrow().next.clone();
        result += &current.clone().unwrap().clone().as_ref().borrow().value.to_string();
    }
    return result;
}

fn solution_part_2(filename: &str, rounds: i32) -> i64 {
    let (nodes, head) = get_input_data(filename, 1000000);
    let mut next = head.clone();
    for _i in 0..rounds {
        next = cycle(&nodes, &next, 1000000);
    }
    let one = nodes.get(&1).unwrap().to_owned();
    let next = one.as_ref().borrow().next.clone();
    let next2 = next.clone().unwrap().as_ref().borrow().next.clone();
    info!("{:?}", next.clone().unwrap().as_ref().borrow().value);
    info!("{:?}", next2.clone().unwrap().as_ref().borrow().value);
    return (next.unwrap().as_ref().borrow().value as i64) *
        (next2.unwrap().as_ref().borrow().value as i64);
}

fn main() {
    env_logger::init();
    info!("{:?}", solution_part_1("inputData.txt", 100));
    info!("{:?}", solution_part_2("inputData.txt", 10000000));
}
