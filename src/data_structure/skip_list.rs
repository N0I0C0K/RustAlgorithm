use std::{marker::PhantomData, ptr::NonNull};

use rand::{rngs::ThreadRng, thread_rng, Rng};

type Node<T> = NonNull<SkipListNode<T>>;

#[derive(Debug)]
struct SkipListNode<T> {
    forwards: Vec<Option<Node<T>>>,
    val: T,
}

impl<T> SkipListNode<T> {
    fn new(val: T, height: usize) -> Self {
        let t: Vec<Option<Node<T>>> = (0..height).map(|_| None::<Node<T>>).collect();
        SkipListNode {
            forwards: t,
            val: val,
        }
    }

    fn new_ptr(val: T, height: usize) -> Node<T> {
        let mut boxed = Box::new(Self::new(val, height));

        let ptr = NonNull::new(boxed.as_mut()).expect("Create Node failed");
        std::mem::forget(boxed);
        return ptr;
    }
}

pub struct SkipList<T> {
    max_height: usize,
    rand: ThreadRng,
    height: usize,
    head: Node<T>,
    len: usize,
}

impl<T> SkipList<T>
where
    T: Default,
{
    fn new(max_height: Option<usize>) -> Self {
        let max_height = max_height.unwrap_or(32);
        SkipList {
            head: SkipListNode::new_ptr(T::default(), max_height),
            max_height,
            rand: thread_rng(),
            height: 0,
            len: 0,
        }
    }

    fn randomLevel(&mut self) -> usize {
        let mut level: usize = 1;
        while self.rand.gen_range(0..100) < 30 {
            level += 1;
        }
        return level.min(self.max_height);
    }

    fn contain(&self, val: T) -> bool
    where
        T: PartialEq + PartialOrd,
    {
        if self.height == 0 {
            return false;
        }
        let mut node = unsafe { self.head.as_ref() };
        for i in (0..self.height).rev() {
            loop {
                if node.forwards[i].is_none() {
                    break;
                }
                let next_node = unsafe { node.forwards[i].as_ref().unwrap().as_ref() };
                if next_node.val > val {
                    break;
                }
                node = next_node;
            }
        }
        return node.val == val;
    }

    fn insert(&mut self, val: T)
    where
        T: PartialOrd,
    {
        let mut node = unsafe { self.head.as_mut() };
        let mut updates: Vec<Option<Node<T>>> = vec![None; self.height + 1];
        for i in (0..self.height).rev() {
            loop {
                if node.forwards[i].is_none() {
                    break;
                }
                let next_node = unsafe { node.forwards[i].as_mut().unwrap().as_mut() };
                if next_node.val > val {
                    break;
                }
                node = next_node;
            }
            updates[i] = NonNull::new(node);
        }

        let mut level = self.randomLevel();
        if level > self.height {
            self.height += 1;
            level = self.height;
            updates[level - 1] = Some(self.head.clone());
        }

        let mut new_node = SkipListNode::new_ptr(val, level);
        let new_node_ref = unsafe { new_node.as_mut() };
        for i in (0..level).rev() {
            let p = unsafe { updates[i].unwrap().as_mut() };
            new_node_ref.forwards[i] = p.forwards[i].clone();
            p.forwards[i] = Some(new_node.clone());
        }
        self.len += 1;
    }

    fn len(&self) -> usize {
        self.len
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter {
            cur: self.head.clone(),
            marker: PhantomData,
        }
    }
}

struct Iter<'a, T: 'a> {
    cur: Node<T>,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let node_ref = unsafe { self.cur.as_ref() };

        if let Some(next_node) = node_ref.forwards[0] {
            let next_node_ref = unsafe { next_node.as_ref() };
            self.cur = next_node.clone();
            return Some(&next_node_ref.val);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::SkipList;

    #[test]
    fn test1() {
        let mut list = SkipList::<i32>::new(Some(16));
        list.insert(10);
        for i in (0..1000).step_by(2) {
            list.insert(i);
        }
        assert_eq!(list.contain(10), true);
        assert_eq!(list.contain(20021), false);
        assert_eq!(list.contain(3), false);
        assert_eq!(list.len(), 501);
        let h = list.height;
        println!("skip list height: {h}");
    }

    #[test]
    fn test2() {
        let mut list = SkipList::<i32>::new(Some(16));

        for i in (0..20).rev() {
            list.insert(i);
        }
        let exceped_list = (0..20).collect::<Vec<i32>>();

        let list = list.iter().map(|x| *x).collect::<Vec<i32>>();

        assert_eq!(exceped_list, list);
    }
}
