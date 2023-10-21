use std::{marker::PhantomData, ptr::NonNull};

use rand::{rngs::ThreadRng, thread_rng, Rng};

type Node<K, V> = NonNull<SkipListNode<K, V>>;

struct SkipListNode<K, V> {
    key: K,
    val: V,
    forwards: Vec<Option<Node<K, V>>>,
}

impl<K, V> SkipListNode<K, V> {
    pub fn new(key: K, val: V, height: usize) -> Self {
        let forwards = (0..height).map(|_| None::<Node<K, V>>).collect();
        SkipListNode {
            key: key,
            val: val,
            forwards: forwards,
        }
    }

    pub fn new_ptr(key: K, val: V, height: usize) -> Node<K, V> {
        let mut boxed = Box::new(Self::new(key, val, height));
        let ptr = NonNull::new(boxed.as_mut()).expect("failed to create node");
        std::mem::forget(boxed);
        return ptr;
    }
}

struct SkipListMap<K, V> {
    head: Node<K, V>,
    len: usize,
    rand: ThreadRng,
    max_height: usize,
    height: usize,
}

impl<K, V> SkipListMap<K, V> {
    fn new(max_height: usize) -> Self
    where
        K: Default,
        V: Default,
    {
        SkipListMap {
            head: SkipListNode::new_ptr(K::default(), V::default(), max_height),
            len: 0,
            rand: thread_rng(),
            max_height: max_height,
            height: 0,
        }
    }

    fn random_level(&mut self) -> usize {
        let mut level: usize = 1;
        while self.rand.gen_range(0..100) < 30 {
            level += 1;
        }
        return level.min(self.max_height);
    }

    fn lower_bound(&self, key: &K) -> Option<Node<K, V>>
    where
        K: PartialOrd,
    {
        if self.height == 0 {
            return None;
        }
        let mut node = unsafe { self.head.as_ref() };
        let mut iter_node = &self.head;

        for i in (0..self.height).rev() {
            loop {
                if node.forwards[i].is_none() {
                    break;
                }
                let next_node_ref = unsafe { node.forwards[i].as_ref().unwrap().as_ref() };

                iter_node = node.forwards[i].as_ref().unwrap();
                node = next_node_ref;
                if next_node_ref.key >= *key {
                    break;
                }
            }
        }

        if node.key < *key {
            return None;
        }

        return Some(iter_node.clone());
    }

    fn upper_bound(&self, key: &K) -> Option<Node<K, V>>
    where
        K: PartialOrd,
    {
        if self.height == 0 {
            return None;
        }
        let mut node = unsafe { self.head.as_ref() };
        let mut iter_node = &self.head;

        for i in (0..self.height).rev() {
            loop {
                if node.forwards[i].is_none() {
                    break;
                }
                let next_node_ref = unsafe { node.forwards[i].as_ref().unwrap().as_ref() };
                node = next_node_ref;
                iter_node = node.forwards[i].as_ref().unwrap();
                if next_node_ref.key > *key {
                    break;
                }
            }
        }

        if node.key <= *key {
            return None;
        }

        return Some(iter_node.clone());
    }

    fn contian(&self, key: K) -> bool
    where
        K: PartialOrd,
    {
        let it = self.lower_bound(&key);
        return it.is_some_and(|x| {
            let t_node = unsafe { x.as_ref() };
            return t_node.key == key;
        });
    }

    fn get_key(&self, key: K) -> Option<&V>
    where
        K: PartialOrd,
    {
        let it = self.lower_bound(&key);

        return it.map_or(None, |x| {
            let t_node = unsafe { x.as_ref() };
            return Some(&t_node.val);
        });
    }

    fn insert(&mut self, key: K, val: V)
    where
        K: PartialOrd,
    {
        let mut node = unsafe { self.head.as_mut() };
        let mut updates: Vec<Option<Node<K, V>>> = vec![None; self.height + 1];

        for i in (0..self.height).rev() {
            loop {
                if node.forwards[i].is_none() {
                    break;
                }
                let next_node_ref = unsafe { node.forwards[i].as_mut().unwrap().as_mut() };
                if next_node_ref.key > key {
                    break;
                }
                node = next_node_ref;
            }
            updates[i] = NonNull::new(node);
        }

        if node.key == key {
            // update node
            node.val = val;
            return;
        }

        let mut level = self.random_level();

        if level > self.height {
            self.height += 1;
            level = self.height;
            updates[level - 1] = Some(self.head.clone());
        }

        let mut new_node = SkipListNode::new_ptr(key, val, level);
        let new_node_ref = unsafe { new_node.as_mut() };

        for i in (0..level).rev() {
            let t_node = unsafe { updates[i].unwrap().as_mut() };
            new_node_ref.forwards[i] = t_node.forwards[i].clone();
            t_node.forwards[i] = Some(new_node.clone());
        }

        self.len += 1;
    }
}

#[cfg(test)]
mod test {
    use super::SkipListMap;

    #[test]
    fn test() {
        let mut map: SkipListMap<i32, i32> = SkipListMap::new(16);
        map.insert(10, 10);
        map.insert(2, 1);
        assert!(map.contian(10));
        assert!(map.contian(2));
        assert_eq!(map.get_key(2), Some(&1));
        assert_eq!(map.get_key(100), None);
    }
}
