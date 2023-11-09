use rand::{self, distributions::Open01, Rng};
use std::{cmp::Ordering, ptr::NonNull};
type PNode<T> = NonNull<Node<T>>;

struct Node<T> {
    val: u32,
    key: T,
    cnt: usize,
    size: usize,
    left: Option<PNode<T>>,
    right: Option<PNode<T>>,
}

enum ChildState {
    No,
    Left,
    Right,
}

impl<T> Node<T> {
    pub fn new(key: T, val: u32) -> Self {
        Node {
            val,
            key,
            cnt: 1,
            size: 1,
            left: None,
            right: None,
        }
    }

    pub fn new_ptr(key: T, val: u32) -> PNode<T> {
        let mut t = Box::new(Self::new(key, val));
        let ptr = NonNull::new(t.as_mut()).expect("null ptr");
        std::mem::forget(t);
        ptr
    }

    fn update_size(&mut self) {
        let mut res: usize = self.cnt;
        self.left.and_then(|x| unsafe {
            res += x.as_ref().size;
            None::<T>
        });

        self.right.and_then(|x| unsafe {
            res += x.as_ref().size;
            None::<T>
        });
        self.size = res;
    }

    fn right_rotate(&mut self) -> PNode<T> {
        let mut left = self.left.clone().unwrap();
        let lright = unsafe { left.as_ref().right.clone() };

        self.left = lright;
        unsafe {
            left.as_mut().right = NonNull::new(self);
        };

        self.update_size();
        unsafe {
            left.as_mut().update_size();
        }

        left
    }

    fn left_rotate(&mut self) -> PNode<T> {
        let mut right = self.right.clone().unwrap();
        let rl = unsafe { right.as_ref().left.clone() };

        self.right = rl;
        unsafe {
            right.as_mut().left = NonNull::new(self);
        }

        self.update_size();
        unsafe {
            right.as_mut().update_size();
        }
        right
    }

    /// only use for remove function
    /// when have just no or one side, return the current side
    /// otherwise, return the small side
    fn child_state(&self) -> ChildState {
        let mut r: u8 = 0;
        if self.left.is_some() {
            r |= 1;
        }
        if self.right.is_some() {
            r |= 2;
        }
        return match r {
            0 => ChildState::No,
            1 => ChildState::Left,
            2 => ChildState::Right,
            3 => unsafe {
                if self.left.as_ref().unwrap().as_ref().val
                    <= self.right.as_ref().unwrap().as_ref().val
                {
                    ChildState::Left
                } else {
                    ChildState::Right
                }
            },
            _ => ChildState::No,
        };
    }

    fn free(node: PNode<T>) {
        unsafe {
            let boxed = Box::from_raw(node.as_ptr());
            std::mem::drop(boxed);
        }
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        if let Some(left) = self.left {
            Node::free(left);
        }
        if let Some(right) = self.right {
            Node::free(right);
        }
    }
}

struct Treap<T> {
    head: Option<PNode<T>>,
    rand: rand::rngs::ThreadRng,
}

impl<T> Treap<T> {
    pub fn new() -> Self {
        Treap {
            head: None,
            rand: rand::thread_rng(),
        }
    }

    fn _insert(&mut self, mut node: Option<PNode<T>>, key: T) -> PNode<T>
    where
        T: PartialOrd,
    {
        if node.is_none() {
            return Node::new_ptr(key, self.rand.gen_range(0..u32::MAX));
        }

        let mut pnode = unsafe { node.as_mut().unwrap().as_mut() };
        pnode.size += 1;
        match pnode.key.partial_cmp(&key).unwrap() {
            Ordering::Less => {
                pnode.right = Some(self._insert(pnode.right, key));
                return pnode
                    .right
                    .and_then(|x| unsafe {
                        if x.as_ref().val < pnode.val {
                            return Some(pnode.left_rotate());
                        }
                        None::<NonNull<Node<T>>>
                    })
                    .unwrap_or(node.unwrap());
            }
            Ordering::Greater => {
                pnode.left = Some(self._insert(pnode.left, key));
                return pnode
                    .left
                    .and_then(|x| unsafe {
                        if x.as_ref().val < pnode.val {
                            return Some(pnode.right_rotate());
                        }
                        None::<NonNull<Node<T>>>
                    })
                    .unwrap_or(node.unwrap());
            }
            Ordering::Equal => {
                pnode.cnt += 1;
                return node.unwrap();
            }
        }
    }

    pub fn insert(&mut self, key: T)
    where
        T: PartialOrd,
    {
        self.head = Some(self._insert(self.head.clone(), key));
    }

    fn _remove(&mut self, mut node: Option<PNode<T>>, key: T) -> Option<PNode<T>>
    where
        T: PartialOrd,
    {
        if node.is_none() {
            return None;
        }

        let mut pnode = unsafe { node.as_mut().unwrap().as_mut() };

        match pnode.key.partial_cmp(&key).unwrap() {
            Ordering::Less => {
                pnode.right = self._remove(pnode.right, key);
                pnode.update_size();
                return node;
            }
            Ordering::Greater => {
                pnode.left = self._remove(pnode.left, key);
                pnode.update_size();
                return node;
            }
            Ordering::Equal => {
                if pnode.cnt > 1 {
                    pnode.cnt -= 1;
                    pnode.size -= 1;
                    return node;
                }

                match pnode.child_state() {
                    ChildState::No => {
                        Node::free(node.unwrap());
                        return None;
                    }
                    ChildState::Left => {
                        let mut nnode = pnode.right_rotate();
                        unsafe {
                            let p_nnode = nnode.as_mut();
                            p_nnode.right = self._remove(p_nnode.right, key);
                            p_nnode.update_size();
                        };
                        return Some(nnode);
                    }
                    ChildState::Right => {
                        let mut nnode = pnode.left_rotate();
                        unsafe {
                            let p_nnode = nnode.as_mut();
                            p_nnode.left = self._remove(p_nnode.left, key);
                            p_nnode.update_size();
                        };
                        return Some(nnode);
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, key: T)
    where
        T: PartialOrd,
    {
        self.head = self._remove(self.head, key);
    }

    pub fn size(&self) -> usize {
        self.head
            .and_then(|x| unsafe { Some(x.as_ref().size) })
            .unwrap_or(0)
    }

    pub fn lower_bound(&self, key: T) -> Option<&T>
    where
        T: PartialOrd,
    {
        let mut res: Option<&T> = None;
        let mut node = self.head.as_ref();
        while let Some(pnode) = node {
            let ref_node = unsafe { pnode.as_ref() };
            if ref_node.key < key {
                node = ref_node.right.as_ref();
            } else {
                res = Some(&ref_node.key);
                node = ref_node.left.as_ref();
            }
        }

        res
    }

    pub fn upper_bound(&self, key: T) -> Option<&T>
    where
        T: PartialOrd,
    {
        let mut res: Option<&T> = None;
        let mut node = self.head.as_ref();
        while let Some(pnode) = node {
            let ref_node = unsafe { pnode.as_ref() };
            if ref_node.key <= key {
                node = ref_node.right.as_ref();
            } else {
                res = Some(&ref_node.key);
                node = ref_node.left.as_ref();
            }
        }

        res
    }
}

impl<T> Drop for Treap<T> {
    fn drop(&mut self) {
        Node::free(self.head.unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::{Node, Treap};

    #[test]
    fn test_drop() {
        let mut head = Node::new(10, 10);
        let mut left = Node::new_ptr(10, 10);
        let mut right = Node::new_ptr(10, 10);

        head.left = Some(left);
        unsafe {
            left.as_mut().left = Some(right);
        }

        std::mem::drop(head);
    }

    #[test]
    fn test_1() {
        let mut treap = Treap::<i32>::new();
        treap.insert(10);
        treap.insert(10);
        treap.insert(10);

        treap.insert(1);
        treap.insert(3);
        treap.insert(-10);
        treap.insert(12);
        treap.insert(100);
        treap.insert(23);
        assert_eq!(treap.size(), 9);
        treap.remove(10);
        assert_eq!(treap.size(), 8);
        treap.remove(10);
        assert_eq!(treap.size(), 7);

        treap.remove(-10);
        assert_eq!(treap.size(), 6);

        treap.remove(100);
        assert_eq!(treap.size(), 5);
    }

    #[test]
    fn test_2() {
        let mut treap = Treap::<i32>::new();
        treap.insert(10);
        treap.insert(10);
        treap.insert(10);

        treap.insert(1);
        treap.insert(3);
        treap.insert(-10);
        treap.insert(12);
        treap.insert(100);
        treap.insert(23);
        assert_eq!(treap.lower_bound(-1000), Some(&-10));
        assert_eq!(treap.lower_bound(1000), None);
        assert_eq!(treap.lower_bound(1), Some(&1));

        treap.remove(1);
        assert_eq!(treap.lower_bound(1), Some(&3));

        assert_eq!(treap.lower_bound(10), Some(&10));
        treap.remove(10);
        treap.remove(10);
        treap.remove(10);
        assert_eq!(treap.lower_bound(10), Some(&12));
    }
}
