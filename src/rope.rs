pub struct RopeNode {
    left: Option<Box<RopeNode>>,
    right: Option<Box<RopeNode>>,
    weight: usize,
    contents: Option<String>,
}

impl RopeNode {
    pub fn new(s: String) -> RopeNode {
        RopeNode {
            left: None,
            right: None,
            weight: s.len(),
            contents: Some(s.clone()),
        }
    }

    // return the character at this index
    pub fn index(&self, i: usize) -> Option<char> {
        self.find_node(i)
            .and_then(|(ref node, index)| node.contents.as_ref().and_then(|c| c.chars().nth(index)))
    }

    // given rope a, and rope b, return a new rope a + b
    // this is constant time.
    pub fn concat(left: RopeNode, right: RopeNode) -> RopeNode {
        let weight = left.weight + right.weight;
        RopeNode {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            weight: weight,
            contents: None,
        }
    }

    // in order to make life easier, users don't need to check i
    // we also return the reduced index for this particular node
    fn find_node(&self, i: usize) -> Option<(&RopeNode, usize)> {
        if i >= self.weight {
            None
        } else if self.weight < i {
            self.right.as_ref().and_then(|right| right.find_node(i - self.weight))
        } else {
            match self.left {
                Some(ref left) => Some((left, i)),
                None => Some((self, i)),
            }
        }
    }

    // return two ropes, split at character i
    pub fn split(&self, i: usize) -> (RopeNode, RopeNode) {
        // find the node at character i
        let node = self.find_node(i);

        // if i is not at the end of the string
        let new_node = if i < node.contents.unwrap().length() {
            // split it into a new node with two children
            let contents = node.contents.unwrap();
            let (contents_left,contents_right) = contents.split_at(i);

            let left_node = RopeNode::new(String::from(contents_left));
            let right_node = RopeNode::new(String::from(contents_right));
            RopeNode::concat(left_node,right_node);
        } else {
            // else just use the node
            node
        }

        // remove any right links to subtrees covering characters past position i
        // subtracting their weights from the parent nodes
        // concatenate the right links
        // rebalance the left tree

        (RopeNode::new(String::from("")), RopeNode::new(String::from("")))
    }

    // immutable, return a new value
    pub fn insert(&self, i: usize, rope: RopeNode) -> RopeNode {
        let (before, after) = self.split(i);
        let before_concat = RopeNode::concat(before, rope);
        RopeNode::concat(before_concat, after)
    }

    // delete the string starting at i, and going to i + len
    pub fn delete(&self, i: usize, len: usize) -> RopeNode {
        let (before, _) = self.split(i);
        let (_, after) = self.split(i + len);
        RopeNode::concat(before, after)
    }

    // TODO
    // report the string from i to i + len
    pub fn report(&self, i: usize, len: usize) -> RopeNode {
        RopeNode::new(String::from(""))
    }

    // return the length of a rope
    pub fn length(&self) -> usize {
        self.weight
    }
}

#[test]
pub fn test_index() {
    let node = RopeNode::new(String::from("a new rope node"));
    // test that a valid bound works
    assert!(node.index(0) == Some('a'));
    assert!(node.index(14) == Some('e'));

    // test that an invalid bound fails
    assert!(node.index(15) == None);
}

#[test]
pub fn test_concat() {
    let left = RopeNode::new(String::from("left"));
    let right = RopeNode::new(String::from("right"));

    let concat = RopeNode::concat(left,right);

    // test that size was updated correctly
    assert!(concat.length() == 9);
}
