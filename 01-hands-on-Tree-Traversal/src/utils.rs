// You can check if a binary tree is balanced by calculating the height of its left and right
// subtrees for each node and ensuring that the height difference between the left and right
// subtrees is at most one for every node in the tree. You can use a recursive function to
// perform this check.
fn is_balanced(root: &Option<Box<TreeNode>>) -> bool {
    fn check_height(node: &Option<Box<TreeNode>>) -> Option<i32> {
        match node {
            Some(node) => {
                let left_height = check_height(&node.left);
                let right_height = check_height(&node.right);

                if let (Some(left), Some(right)) = (left_height, right_height) {
                    let height_diff = i32::abs(left - right);
                    if height_diff <= 1 {
                        return Some(i32::max(left, right) + 1);
                    }
                }

                None
            }
            None => Some(0),  // Height of an empty tree is 0.
        }
    }

    check_height(root).is_some()
}

// To check if a binary tree is a max-heap, you can use a recursive algorithm to ensure that every
// node in the tree satisfies the max-heap property, which means that the value of each node should
// be greater than or equal to the values of its children.

fn is_max_heap(root: &Option<Box<TreeNode>>) -> bool {
    fn check_max_heap(node: &Option<Box<TreeNode>>) -> Option<i32> {
        match node {
            Some(node) => {
                let left_value = node.left.as_ref().map(|child| child.value);
                let right_value = node.right.as_ref().map(|child| child.value);

                match (left_value, right_value) {
                    (Some(left), Some(right)) => {
                        if node.value >= cmp::max(left, right) {
                            let left_height = check_max_heap(&node.left);
                            let right_height = check_max_heap(&node.right);
                            if left_height.is_some() && right_height.is_some() {
                                return Some(cmp::max(left_height.unwrap(), right_height.unwrap()) + 1);
                            }
                        }
                    }
                    _ => return None,
                }
            }
            None => return Some(0), // An empty tree is a max-heap.
        }
        None
    }

    check_max_heap(root).is_some()
}
