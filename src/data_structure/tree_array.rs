use std::ops::{Add, AddAssign};

struct TreeArray<T> {
    tree: Vec<T>,
}

impl<T> TreeArray<T>
where
    T: Add<Output = T> + Default + AddAssign + PartialOrd + Copy,
{
    pub fn lowbit(x: i32) -> i32 {
        x & -x
    }

    pub fn query(&mut self, r: usize) -> Option<T> {
        let mut i = (r + 1) as i32;
        let mut res = T::default();
        while i > 0 {
            res += self.tree[i as usize];
            i -= Self::lowbit(i);
        }
        Some(res)
    }

    pub fn add(&mut self, idx: usize, val: T) {
        let idx = (idx + 1) as i32;
        let mut i = 1i32;
        while i < self.tree.len() as i32 {
            self.tree[i as usize] += val;
            i += Self::lowbit(i);
        }
    }

    pub fn new(nums: &Vec<T>) -> Self {
        let mut tree = vec![T::default(); nums.len() + 1];
        let tree_len = tree.len();
        for i in 1..=nums.len() {
            tree[i] += nums[i - 1];
            let j = (i as i32 + Self::lowbit(i as i32)) as usize;
            if j < tree_len {
                let p = tree[i];
                tree[j] += p;
            }
        }
        TreeArray { tree }
    }
}

#[cfg(test)]
mod test {
    use super::TreeArray;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 4, 5];
        let mut arr_tree = TreeArray::new(&nums);
        assert_eq!(arr_tree.query(0), Some(1));
        assert_eq!(arr_tree.query(4), Some(15));
    }
}
