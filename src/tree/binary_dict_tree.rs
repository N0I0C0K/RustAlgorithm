use std::ops::{BitAnd, Shl};

struct DicTree {
    tree: Vec<(usize, usize)>,
    idx: usize,
}

impl DicTree {
    pub fn new(nums: &Vec<i32>) -> Self {
        let mut sf = DicTree {
            tree: vec![(0, 0); nums.len() * 32],
            idx: 0,
        };
        for num in nums {
            sf.add(*num);
        }
        sf
    }

    pub fn add(&mut self, num: i32) {
        let mut h: usize = 0;
        for i in (0..32).rev() {
            let bit = num & (1 << i);
            if bit == 0 {
                if self.tree[h].0 == 0 {
                    self.idx += 1;
                    self.tree[h].0 = self.idx;
                }
                h = self.tree[h].0;
            } else {
                if self.tree[h].1 == 0 {
                    self.idx += 1;
                    self.tree[h].1 = self.idx;
                }
                h = self.tree[h].1;
            }
        }
    }

    pub fn query(&self, num: i32) -> i32 {
        let mut h: usize = 0;
        let mut res = 0;
        for i in (0..32).rev() {
            let bit = num & (1 << i);
            if bit == 0 {
                if self.tree[h].1 != 0 {
                    res |= 1 << i;
                    h = self.tree[h].1;
                } else {
                    h = self.tree[h].0;
                }
            } else {
                if self.tree[h].0 != 0 {
                    res |= 1 << i;
                    h = self.tree[h].0;
                } else {
                    h = self.tree[h].1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::DicTree;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 4];
        let mut dic = DicTree::new(&nums);
        assert_eq!(dic.query(5), 7);
    }
}
