// Assuming a basic definition of a binary tree node
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => 1 + max_depth(node.left) .max(max_depth(node.right)),
        None => 0,
    }
}
