// In the code snippet below, we provide a (limited) set of tests for the sum method.
// This code also shows how to construct a binary tree using our implementation.
// To ensure the robustness of your solutions, we strongly recommend adding a comprehensive suite of tests

// These additional test cases cover a range of scenarios, including an empty tree, negative values,
// and large values.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);

        // Test an empty tree
        let empty_tree = Tree::new();
        assert_eq!(empty_tree.sum(), 0);

        // Test a tree with negative values
        let mut negative_tree = Tree::with_root(-5);
        negative_tree.add_node(0, -2, true); // id 1
        negative_tree.add_node(0, -8, false); // id 2
        assert_eq!(negative_tree.sum(), -15);

        // Test a tree with large values
        let mut large_tree = Tree::with_root(1000);
        large_tree.add_node(0, 500, true); // id 1
        large_tree.add_node(0, 2000, false); // id 2
        assert_eq!(large_tree.sum(), 2500);

        // Add more nodes and test the sum
        large_tree.add_node(1, 300, false); // id 3
        large_tree.add_node(2, 700, true); // id 4
        assert_eq!(large_tree.sum(), 4500);
    }
}