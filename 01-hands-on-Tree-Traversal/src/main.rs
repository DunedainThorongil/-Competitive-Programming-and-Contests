struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

// Struttura risultato
struct Result {
    is_balanced: bool,
    height: i32,
}

impl Result {
    fn new(is_balanced: bool, height: i32) -> Self {
        Self {
            is_balanced,
            height,
        }
    }
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert_eq!(self.nodes[parent_id].id_left, None, "Parent node has the left child already set");
        } else {
            assert_eq!(self.nodes[parent_id].id_right, None, "Parent node has the right child already set");
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    // ###### Exercise #1 Write a method to check if the binary tree is a Binary Search Tree.

    // In a BST, for any given node:
    //  1) All nodes in its left subtree should have values less than the node's value;
    //  2) All nodes in its right subtree should have values greater than the node's value.
    //
    // (An empty tree is a BST)
    fn is_abr(&self, node_id: Option<usize>, min: u32, max: u32) -> bool {
        match node_id {
            None => true,
            Some(id) => {
                let node = &self.nodes[id];
                if node.key < min || node.key > max {
                    return false;
                }
                self.is_abr(node.id_left, min, node.key - 1)
                    && self.is_abr(node.id_right, node.key + 1, max)
            }
        }
    }

    // Exercise #2
    // Write a method to check if the binary tree is balanced.
    //
    // A tree is considered balanced if, for each of its nodes, the heights of its left and right subtrees differ by at most one.
    fn is_balanced(&self, node_id: Option<usize>) -> Result {
        match node_id {
            None => Result::new(true, -1), // Albero vuoto è bilanciato
            Some(id) => {
                let node = &self.nodes[id];

                let left_result = self.is_balanced(node.id_left);
                let right_result = self.is_balanced(node.id_right);

                if left_result.is_balanced
                    && right_result.is_balanced
                    && (left_result.height - right_result.height).abs() <= 1
                {
                    Result::new(true, left_result.height.max(right_result.height) + 1)
                } else {
                    Result::new(false, -1)
                }
            }
        }
    }

    // Exercise #3
    // Write a method to check if the binary tree is a max-heap.
    //
    // A max-heap is a complete binary tree in which every node satisfies the max-heap property.
    // A node satisfies the max-heap property if its key is greater than or equal to the keys of its children
    fn is_max_heap(&self, node_id: Option<usize>) -> bool {
        match node_id {
            None => true, // Albero vuoto è un max-heap
            Some(id) => {
                let node = &self.nodes[id];
                let left_id = node.id_left;
                let right_id = node.id_right;

                if let Some(left) = left_id {
                    if self.nodes[left].key > node.key {
                        return false;
                    }
                }

                if let Some(right) = right_id {
                    if self.nodes[right].key > node.key {
                        return false;
                    }
                }

                self.is_max_heap(left_id) && self.is_max_heap(right_id)
            }
        }
    }

}


fn main() {
    let mut tree = Tree::with_root(10);

    let node1_id = tree.nodes.len();
    tree.nodes.push(Node::new(5));
    tree.nodes[0].id_left = Some(node1_id);

    let node2_id = tree.nodes.len();
    tree.nodes.push(Node::new(15));
    tree.nodes[0].id_right = Some(node2_id);

    let result = tree.is_abr(Some(0), u32::MIN, u32::MAX);

    if result {
        println!("L'albero è un ABR.");
    } else {
        println!("L'albero NON è un ABR.");
    }

    let node1_id = tree.nodes.len();
    tree.nodes.push(Node::new(5));
    tree.nodes[0].id_left = Some(node1_id);

    let node2_id = tree.nodes.len();
    tree.nodes.push(Node::new(22));
    tree.nodes[0].id_right = Some(node2_id);

    let result = tree.is_balanced(Some(0));

    if result.is_balanced {
        println!("L'albero è bilanciato.");
    } else {
        println!("L'albero NON è bilanciato.");
    }

    let node1_id = tree.nodes.len();
    tree.nodes.push(Node::new(5));
    tree.nodes[0].id_left = Some(node1_id);

    let node2_id = tree.nodes.len();
    tree.nodes.push(Node::new(15));
    tree.nodes[0].id_right = Some(node2_id);

    if tree.is_max_heap(Some(0)) {
        println!("L'albero è un max-heap.");
    } else {
        println!("L'albero NON è un max-heap.");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);
        assert_eq!(tree.is_bst(), true);
        assert_eq!(tree.is_balanced(None), true);
        assert_eq!(tree.is_maxheap(), true);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);
        assert_eq!(tree.is_bst(), true);
        assert_eq!(tree.is_balanced(None), true);
        assert_eq!(tree.is_maxheap(), false);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
        assert_eq!(tree.is_bst(), true);
        assert_eq!(tree.is_balanced(None), true);
        assert_eq!(tree.is_maxheap(), false);
        assert_eq!(tree.count_nodes(Some(0)), 5);
    }
}

