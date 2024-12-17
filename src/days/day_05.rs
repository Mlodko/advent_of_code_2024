use std::{collections::{HashMap, VecDeque}, fs, io::Error};

struct OrderingRule {
    before : u32,
    after : u32
}

impl OrderingRule {
    fn from_text(text: &str) -> Option<Self> {
        let split_text: Vec<&str> = text.split('|').collect();
        if split_text.len() != 2 {
            return None;
        }

        let before : u32 = split_text.first()?.parse().ok()?;
        let after : u32 = split_text.get(1)?.parse().ok()?;
        Some(OrderingRule { before, after })
    }

    fn is_in_right_order(&self, nums : &[u32]) -> bool {
        let before_found = nums.iter().position(|num| num == &self.before);
        let after_found = nums.iter().position(|num| num == &self.after);

        if let (Some(before_index), Some(after_index)) = (before_found, after_found) {
            before_index < after_index
        } else {
            true
        }
    }
}

pub fn solve_part_1(path_to_input : &str) -> Result<u32, Error> {
    let input = fs::read_to_string(path_to_input)?;
    let (rules, orders) = parse_input(&input);
    let result : u32 = orders.iter()
        .filter(|nums| rules.iter().all(|rule| rule.is_in_right_order(nums)))
        .filter_map(|order| order.get(order.len() / 2).cloned())
        .sum();
    Ok(result)
}

pub fn solve_part_2(path_to_input : &str) -> Result<u32, Error> {
    let input = fs::read_to_string(path_to_input)?;
    let (rules, orders) = parse_input(&input);

    // Collect orders that violate at least one rule
    let violating_orders: Vec<&Vec<u32>> = orders.iter()
        .filter(|nums| rules.iter().any(|rule| !rule.is_in_right_order(nums)))
        .collect();
    println!("Number of orders violating at least one rule: {}", violating_orders.len());

    // Collect orders that can be topologically sorted
    let sorted_orders: Vec<Vec<u32>> = violating_orders.iter()
        .filter_map(|nums| topological_sort(&rules, nums))
        .collect();
    println!("Number of orders that can be topologically sorted: {}", sorted_orders.len());

    let result : u32 = sorted_orders.iter()
        .filter_map(|order| order.get(order.len() / 2))
        .sum();


    Ok(result)
}


fn parse_input(input : &str) -> (Vec<OrderingRule>, Vec<Vec<u32>>) {
    // First ordering rules until empty line
    let ordering_rules : Vec<OrderingRule> = input.lines()
        .take_while(|line| !line.is_empty())
        .filter_map(OrderingRule::from_text)
        .collect();

    // Then skip until after empty line
    let orders : Vec<Vec<u32>> = input.lines()
        .skip(ordering_rules.len() + 1)
        .map(|line| {
            line.split(',')
            .filter_map(|str_num| str_num.parse().ok())
            .collect()
        })
        .collect();

    (ordering_rules, orders)
}

fn topological_sort(rules: &[OrderingRule], nums: &[u32]) -> Option<Vec<u32>> {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();

    // Initialize graph and in_degree for each number in nums
    for &num in nums {
        graph.entry(num).or_default();
        in_degree.entry(num).or_insert(0);
    }

    // Populate graph and in_degree based on rules, but only for nodes in nums
    for rule in rules {
        if nums.contains(&rule.before) && nums.contains(&rule.after) {
            graph.entry(rule.before).or_default().push(rule.after);
            *in_degree.entry(rule.after).or_insert(0) += 1;
        }
    }

    // Kahn's algorithm for sorting
    let mut queue: VecDeque<u32> = in_degree.iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&num, _)| num)
        .collect();

    let mut sorted: Vec<u32> = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 && nums.contains(&neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    // Check if all nodes are sorted
    if sorted.len() == nums.len() {
        Some(sorted)
    } else {
        None
    }
}