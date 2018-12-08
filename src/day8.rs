use std::slice::Iter;

pub struct NodeHeader {
    child_count: usize,
    metadata_count: usize,
}

pub struct Node {
    header: NodeHeader,
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn new(iter: &mut Iter<usize>) -> Self {
        let mut node = Node {
            header: NodeHeader {
                child_count: *iter.next().unwrap(),
                metadata_count: *iter.next().unwrap(),
            },
            children: Vec::new(),
            metadata: Vec::new(),
        };

        while node.header.child_count != node.children.len() {
            let child = Node::new(iter);
            node.children.push(child);
        }

        while node.header.metadata_count != node.metadata.len() {
            for _ in 0..node.header.metadata_count {
                node.metadata.push(*iter.next().unwrap());
            }
        }

        node
    }

    fn metadata_sum(&self) -> usize {
        let mut data: Vec<usize> = Vec::new();
        for child in self.children.iter() {
            data.push(child.metadata_sum());
        }
        data.push(self.metadata.iter().sum());
        data.iter().sum()
    }

    fn node_value(&self) -> usize {
        let mut data: Vec<usize> = Vec::new();
        if self.header.child_count > 0 {
            for meta_index in self.metadata.iter() {
                match self.children.get(*meta_index - 1) {
                    Some(child) => {
                        data.push(child.node_value());
                    }
                    None => continue,
                }
            }
        } else {
            data.push(self.metadata.iter().sum());
        }
        data.iter().sum()
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(' ').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let node = Node::new(&mut input.iter());
    node.metadata_sum()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let node = Node::new(&mut input.iter());
    node.node_value()
}

fn solve_a_fast(iter: &mut Iter<usize>) -> usize {
    match (iter.next().unwrap(), iter.next().unwrap()) {
        (child_count, meta_count) => {
            (0..*child_count).map(|_| solve_a_fast(iter)).sum::<usize>()
                + iter.take(*meta_count).sum::<usize>()
        }
    }
}

fn solve_b_fast(iter: &mut Iter<usize>) -> usize {
    match (iter.next().unwrap(), iter.next().unwrap()) {
        (0, meta_count) => iter.take(*meta_count).sum(),
        (child_count, meta_count) => {
            let sums = (0..*child_count)
                .map(|_| solve_b_fast(iter))
                .collect::<Vec<usize>>();
            iter.take(*meta_count).filter_map(|x| sums.get(x - 1)).sum()
        }
    }
}

#[aoc(day8, part1, fast)]
pub fn solve_part1_fast(input: &[usize]) -> usize {
    let mut iter = input.iter();
    solve_a_fast(&mut iter)
}

#[aoc(day8, part2, fast)]
pub fn solve_part2_fast(input: &[usize]) -> usize {
    let mut iter = input.iter();
    solve_b_fast(&mut iter)
}

#[cfg(test)]
mod tests {
    use crate::day8::*;

    #[test]
    fn part1() {
        let input = input_generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        let result = solve_part1(&input);
        assert_eq!(result, 138);
    }

    #[test]
    fn part2() {
        let input = input_generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        let result = solve_part2(&input);
        assert_eq!(result, 66);
    }

    #[test]
    fn part1_fast() {
        let input = input_generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        let result = solve_part1_fast(&input);
        assert_eq!(result, 138);
    }

    #[test]
    fn part2_fast() {
        let input = input_generator("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        let result = solve_part2_fast(&input);
        assert_eq!(result, 66);
    }
}
