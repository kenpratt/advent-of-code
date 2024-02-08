use crate::interface::AoC;

pub struct Day;
impl AoC<Tree, usize, usize> for Day {
    const FILE: &'static str = file!();

    fn parse(input: String) -> Tree {
        let nums = input
            .split_whitespace()
            .flat_map(|s| s.parse::<u8>())
            .collect();
        Tree::from(nums)
    }

    fn part1(tree: &Tree) -> usize {
        tree.iter()
            .map(|node| node.metadata.iter().sum::<u8>() as usize)
            .sum()
    }

    fn part2(tree: &Tree) -> usize {
        tree.value()
    }
}

#[derive(Debug)]
pub struct Tree {
    root: Node,
}

impl Tree {
    fn from(nums: Vec<u8>) -> Self {
        let (root, took) = Node::parse(&nums);
        assert_eq!(took, nums.len());
        Self { root }
    }

    fn iter(&self) -> TreeIterator {
        TreeIterator::new(&self)
    }

    fn value(&self) -> usize {
        self.root.value()
    }
}

#[derive(Debug)]
struct Node {
    children: Vec<Box<Node>>,
    metadata: Vec<u8>,
}

impl Node {
    fn parse(nums: &[u8]) -> (Self, usize) {
        let len = nums.len();
        assert!(len > 2);

        let num_children = nums[0];
        let num_metadata = nums[1];

        // parse each child
        let mut children = vec![];
        let mut curr_i = 2;
        for _ in 0..num_children {
            let (child, took) = Node::parse(&nums[curr_i..]);
            children.push(Box::new(child));
            curr_i += took;
        }

        // grab the metadata
        let metadata_i = curr_i;
        curr_i += num_metadata as usize;
        let metadata = nums[metadata_i..curr_i].iter().cloned().collect();

        let node = Self { children, metadata };
        (node, curr_i)
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum::<u8>() as usize
        } else {
            self.metadata
                .iter()
                .map(|i| match self.children.get((i - 1) as usize) {
                    Some(child) => child.value(),
                    None => 0,
                })
                .sum()
        }
    }
}

struct TreeIterator<'a> {
    // bool indicates whether we've visited the node itself yet
    // usize is index of next child to visit
    stack: Vec<(&'a Node, bool, usize)>,
}

impl<'a> TreeIterator<'a> {
    fn new(tree: &'a Tree) -> Self {
        let stack = vec![(&tree.root, false, 0)];
        Self { stack }
    }
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.last_mut() {
            Some((node, visited_self, child_index)) => {
                if !*visited_self {
                    // haven't returned the node yet - do that first, then go to children
                    *visited_self = true;
                    Some(node)
                } else if *child_index < node.children.len() {
                    // push the next child onto the stack and recur to visit it
                    let child = &node.children[*child_index];
                    *child_index += 1;
                    self.stack.push((child, false, 0));
                    self.next()
                } else {
                    // we've reached the end of this node, pop and recur
                    self.stack.pop();
                    self.next()
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let result = Day::part1(&Day::parse_example_file());
        assert_eq!(result, 138);
    }

    #[test]
    fn test_part1_solution() {
        let result = Day::part1(&Day::parse_input_file());
        assert_eq!(result, 37905);
    }

    #[test]
    fn test_part2_example() {
        let result = Day::part2(&Day::parse_example_file());
        assert_eq!(result, 66);
    }

    #[test]
    fn test_part2_solution() {
        let result = Day::part2(&Day::parse_input_file());
        assert_eq!(result, 33891);
    }
}
