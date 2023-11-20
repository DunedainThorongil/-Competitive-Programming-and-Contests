mod tests;

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

    // To check if a binary tree is a Binary Search Tree (BST), you can use a recursive algorithm that
    // checks if the values in the tree's nodes follow the BST property. In a BST, for any given node:
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
}
