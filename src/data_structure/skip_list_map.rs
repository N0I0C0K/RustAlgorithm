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
        if self.len == 0 || self.height == 0 {
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
                if next_node_ref.key > *key {
                    break;
                }

                iter_node = node.forwards[i].as_ref().unwrap();
                node = next_node_ref;
            }
        }

        if node.key < *key {
            return node.forwards[0].clone();
        }

        return Some(iter_node.clone());
    }

    fn upper_bound(&self, key: &K) -> Option<Node<K, V>>
    where
        K: PartialOrd,
    {
        if self.len == 0 || self.height == 0 {
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
                if next_node_ref.key > *key {
                    break;
                }
                node = next_node_ref;
                iter_node = node.forwards[i].as_ref().unwrap();
            }
            if node.key > *key {
                break;
            }
        }

        if node.key <= *key {
            return node.forwards[0].clone();
        }

        return Some(iter_node.clone());
    }

    fn contain(&self, key: &K) -> bool
    where
        K: PartialOrd,
    {
        let it = self.lower_bound(&key);
        return it.is_some_and(|x| {
            let t_node = unsafe { x.as_ref() };
            return t_node.key == *key && !std::ptr::eq(t_node, unsafe { self.head.as_ref() });
        });
    }

    fn get_key(&self, key: &K) -> Option<&V>
    where
        K: PartialOrd,
    {
        let it = self.lower_bound(key);

        return it.map_or(None, |x| {
            let t_node = unsafe { x.as_ref() };
            if t_node.key != *key || std::ptr::eq(t_node, unsafe { self.head.as_ref() }) {
                return None;
            }

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

        if node.key == key && unsafe { !std::ptr::eq(node, self.head.as_ref()) } {
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

    fn erase(&mut self, key: &K)
    where
        K: PartialOrd,
    {
        let mut node = unsafe { self.head.as_ref() };
        let mut node_iter = &self.head;
        let mut updates: Vec<Option<Node<K, V>>> = vec![None; self.height];

        let mut find = false;
        for i in (0..self.height).rev() {
            loop {
                if node.forwards[i].is_none() {
                    break;
                }
                let next_node_ref = unsafe { node.forwards[i].as_ref().unwrap().as_ref() };
                if next_node_ref.key >= *key {
                    break;
                }
                node_iter = node.forwards[i].as_ref().unwrap();
                node = next_node_ref;
            }

            node.forwards[i].as_ref().and_then(|x| {
                let next_node_ref = unsafe { x.as_ref() };
                if next_node_ref.key == *key {
                    updates[i] = Some(node_iter.clone());
                }
                return Some(true);
            });
        }

        updates
            .iter_mut()
            .enumerate()
            .filter(|(_, x)| x.is_some())
            .for_each(|(lev, node)| {
                let node_ref = unsafe { node.unwrap().as_mut() };
                let next_node = node_ref.forwards[lev].as_mut();
                let next_node_ref = unsafe { next_node.unwrap().as_mut() };
                node_ref.forwards[lev] = next_node_ref.forwards[lev].clone();
            });
        self.len -= 1;
    }

    fn iter_key(&self) -> IterKey<'_, K, V> {
        IterKey {
            cur: self.head.clone(),
            maker: PhantomData,
            maker1: PhantomData,
        }
    }

    fn iter_key_val(&self) -> IterKeyVal<'_, K, V> {
        IterKeyVal {
            cur: self.head.clone(),
            maker: PhantomData,
            maker1: PhantomData,
        }
    }
}

struct IterKeyVal<'a, K, V> {
    cur: Node<K, V>,
    maker: PhantomData<&'a K>,
    maker1: PhantomData<&'a V>,
}

impl<'a, K, V> Iterator for IterKeyVal<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        let cur_ref = unsafe { self.cur.as_ref() };
        if let Some(next) = cur_ref.forwards[0] {
            let next_ref = unsafe { next.as_ref() };
            self.cur = next.clone();
            return Some((&next_ref.key, &next_ref.val));
        }
        None
    }
}

struct IterKey<'a, K, V> {
    cur: Node<K, V>,
    maker: PhantomData<&'a K>,
    maker1: PhantomData<&'a V>,
}

impl<'a, K, V> Iterator for IterKey<'a, K, V> {
    type Item = &'a K;
    fn next(&mut self) -> Option<Self::Item> {
        let cur_ref = unsafe { self.cur.as_ref() };
        if let Some(next) = cur_ref.forwards[0] {
            let next_ref = unsafe { next.as_ref() };
            self.cur = next.clone();
            return Some(&next_ref.key);
        }
        None
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
        map.insert(100, 10);
        assert!(map.contain(&10));

        assert_eq!(map.get_key(&101), None);

        // test modify
        assert_eq!(map.get_key(&10), Some(&10));
        map.insert(10, 101);
        assert_eq!(map.get_key(&10), Some(&101));
    }

    #[test]
    fn test_float_keys() {
        let mut map: SkipListMap<f32, i32> = SkipListMap::new(16);

        // 插入浮点数键值对
        map.insert(5.5, 5);
        map.insert(3.14, 3);

        // 测试包含键的情况
        assert!(map.contain(&5.5));
        assert!(map.contain(&3.14));
        assert!(!map.contain(&999.99)); // 不存在的键

        // 测试获取键对应的值
        assert_eq!(map.get_key(&5.5), Some(&5));
        assert_eq!(map.get_key(&3.14), Some(&3));
        assert_eq!(map.get_key(&999.99), None); // 不存在的键
    }

    #[test]
    fn test_many() {
        let mut map: SkipListMap<i32, i32> = SkipListMap::new(16);
        for i in 0..10000 {
            map.insert(i, i * 2);
        }

        for i in 100..2000 {
            map.erase(&i);
            assert!(!map.contain(&i));
        }
    }

    #[test]
    fn test_del() {
        let mut map: SkipListMap<i32, i32> = SkipListMap::new(16);
        assert!(!map.contain(&i32::default()));
        for i in 0..10 {
            map.insert(i, i * i);
            assert!(map.contain(&i));
        }

        for i in (0..10).rev() {
            map.erase(&i);
        }
        for i in 0..10 {
            assert!(!map.contain(&i));
        }
    }

    #[test]
    fn test_5() {
        let mut map: SkipListMap<i32, i32> = SkipListMap::new(16);
        for i in 10..100 {
            map.insert(i, i);
        }

        for i in 0..8 {
            assert!(!map.contain(&i));
            assert_eq!(map.get_key(&i), None);
        }
    }

    #[test]
    fn test_iter_key() {
        let mut map: SkipListMap<i32, i32> = SkipListMap::new(16);
        for i in -100..100 {
            map.insert(i, i);
        }

        let keys = map.iter_key().map(|x| *x).collect::<Vec<i32>>();
        let except = (-100..100).collect::<Vec<i32>>();
        assert_eq!(keys, except);
    }

    #[test]
    fn test_iter_key_val() {
        let mut map: SkipListMap<i32, i32> = SkipListMap::new(16);
        for i in -100..100 {
            map.insert(i, i);
        }

        let keys_val = map
            .iter_key()
            .map(|x| (*x, *x))
            .collect::<Vec<(i32, i32)>>();
        let except = (-100..100).map(|x| (x, x)).collect::<Vec<(i32, i32)>>();
        assert_eq!(keys_val, except);
    }
}
