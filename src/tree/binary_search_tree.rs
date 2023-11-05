use std::{mem::swap, ptr::NonNull};

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

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
    fn delete(&mut self) -> Option<&TreeNode<T>> {
        None
    }
}

struct BinarySearchTree<T> {
    head: Option<Box<TreeNode<T>>>,
}

impl<T> BinarySearchTree<T>
where
    T: PartialOrd,
{
    fn new() -> Self {
        BinarySearchTree { head: None }
    }

    fn lower_bound(&self, val: &T) -> Option<&TreeNode<T>>
    where
        T: PartialOrd,
    {
        if self.head.is_none() {
            return None;
        }
        let mut t_node = self.head.as_ref();
        let mut res = None::<&TreeNode<T>>;
        while let Some(node) = t_node {
            if *val < node.val {
                res = Some(node);
                t_node = node.left.as_ref();
            } else if *val > node.val {
                res = Some(node);
                t_node = node.right.as_ref();
            } else {
                res = Some(node);
                break;
            }
        }
        res
    }

    fn upper_bound(&self, val: &T) -> Option<&TreeNode<T>>
    where
        T: PartialOrd,
    {
        return self.lower_bound(val).map_or(None, |x| {
            if x.val == *val {
                return match x.right.as_ref() {
                    Some(ri) => Some(ri),
                    None => None,
                };
            }
            return Some(x);
        });
    }

    fn delete(&mut self, target: T) {
        if self.head.is_none() {
            return;
        }
        let node = self.lower_bound(&target);
        if node.is_none() || node.is_some_and(|t| t.val != target) {
            return;
        }
    }

    fn insert(&mut self, val: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(TreeNode::new(val)));
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

    fn contain(&self, val: &T) -> bool
    where
        T: PartialOrd,
    {
        let node = self.lower_bound(val);
        return node.is_some_and(|x| x.val == *val);
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    #[test]
    fn test() {
        let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(!tree.contain(&10));
        tree.insert(10);
        assert!(tree.contain(&10));
        tree.insert(11);
        tree.insert(2);
        tree.insert(12);
        tree.insert(213);
        tree.insert(7);

        let t = tree.lower_bound(&3);

        assert!(t.is_some_and(|x| { x.val == 7 }));

        let t = tree.lower_bound(&11);

        assert!(t.is_some_and(|x| { x.val == 11 }));

        let t = tree.upper_bound(&11);

        assert!(t.is_some_and(|x| { x.val == 12 }));

        let t = tree.upper_bound(&213);

        assert!(t.is_none());

        let t = tree.upper_bound(&-121);

        assert!(t.is_some_and(|x| { x.val == 2 }));
    }
}
