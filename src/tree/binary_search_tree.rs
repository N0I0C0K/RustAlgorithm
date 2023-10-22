struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> Self {
        TreeNode {
            val: val,
            left: None,
            right: None,
        }
    }
}

struct BinarySearchTree<T> {
    head: Option<TreeNode<T>>,
}

impl<T> BinarySearchTree<T>
where
    T: PartialOrd,
{
    fn new() -> Self {
        BinarySearchTree { head: None }
    }

    fn insert(&mut self, val: T) {
        if self.head.is_none() {
            self.head = Some(TreeNode::new(val));
            return;
        }
        let mut node = self.head.as_mut().unwrap();
        loop {
            if val < node.val {
                if node.left.is_none() {
                    node.left = Some(Box::new(TreeNode::new(val)));
                    return;
                } else {
                    node = node.left.as_mut().unwrap();
                }
            } else {
                if node.right.is_none() {
                    node.right = Some(Box::new(TreeNode::new(val)));
                    return;
                } else {
                    node = node.right.as_mut().unwrap();
                }
            }
        }
    }
}
