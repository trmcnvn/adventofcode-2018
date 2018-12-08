use std::slice::Iter;

#[derive(Debug, Default, Clone)]
pub struct NodeHeader {
    child_count: usize,
    metadata_count: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Node {
    header: NodeHeader,
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn build_child(iter: &mut Iter<'_, usize>) -> Node {
    Node {
        header: NodeHeader {
            child_count: *iter.next().unwrap(),
            metadata_count: *iter.next().unwrap(),
        },
        ..Default::default()
    }
}

fn process_tree(root: &mut Node, iter: &mut Iter<'_, usize>) -> Node {
    while root.header.child_count != root.children.len() {
        let child = process_tree(&mut build_child(iter), iter);
        root.children.push(child);
    }
    while root.header.metadata_count != root.metadata.len() {
        for _ in 0..root.header.metadata_count {
            root.metadata.push(*iter.next().unwrap());
        }
    }
    root.clone()
}

fn extract_metadata(node: &Node) -> Vec<usize> {
    let mut data: Vec<usize> = Vec::new();
    for child in node.children.iter() {
        data.extend(extract_metadata(&child));
    }
    data.extend(node.metadata.iter());
    data
}

fn node_value(node: &Node) -> Vec<usize> {
    let mut data: Vec<usize> = Vec::new();
    if node.header.child_count > 0 {
        for meta_index in node.metadata.iter() {
            match node.children.get(*meta_index - 1) {
                Some(child) => {
                    data.push(node_value(&child).iter().sum());
                }
                None => continue,
            }
        }
    } else {
        let x = node.metadata.iter().sum();
        data.push(x);
    }
    data
}

pub fn input_generator(input: &str) -> Node {
    let nums: Vec<usize> = input
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut iter = nums.iter();
    let mut root = Node {
        header: NodeHeader {
            child_count: *iter.next().unwrap(),
            metadata_count: *iter.next().unwrap(),
        },
        ..Default::default()
    };
    process_tree(&mut root, &mut iter)
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> usize {
    let node = input_generator(input);
    extract_metadata(&node).iter().sum()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> usize {
    let node = input_generator(input);
    node_value(&node).iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day8::*;

    #[test]
    fn part1() {
        let result = solve_part1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(result, 138);
    }

    #[test]
    fn part2() {
        let result = solve_part2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(result, 66);
    }
}
