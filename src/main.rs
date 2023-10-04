use std::marker::Sized;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

struct SegTree<'a, T> {
    n: usize,
    nums: &'a Vec<T>,
    tree: Vec<T>,
    lazy: Vec<T>,
}

impl<'a, T> SegTree<'a, T>
where
    T: Default + Sized + Copy + Add<Output = T> + Mul<Output = T> + Eq + AddAssign + From<usize>,
{
    fn new(nums: &'a Vec<T>) -> Self {
        let n = nums.len();

        let mut obj = SegTree {
            n,
            nums,
            lazy: vec![T::default(); (n + 1) * 4],
            tree: vec![T::default(); (n + 1) * 4],
        };
        obj.build(1, n - 1, 1);
        obj
    }

    fn pushup(&mut self, p: usize) {
        self.tree[p] = self.tree[p * 2] + self.tree[p * 2 + 1];
    }

    fn pushdown(&mut self, l: usize, r: usize, p: usize) {
        if l == r || self.lazy[p] == T::default() {
            return;
        }
        let tmp: T = self.lazy[p];
        let m = l + (r - l) / 2;
        self.tree[p * 2] += tmp * T::from(m - l + 1);
        self.tree[p * 2 + 1] += tmp * T::from(r - m);

        self.lazy[p] = T::default();
        self.lazy[p * 2] += tmp;
        self.lazy[p * 2 + 1] += tmp;
    }

    fn build(&mut self, l: usize, r: usize, p: usize) {
        if l == r {
            self.tree[p] = self.nums[l];
            return;
        }
        let m = l + (r - l) / 2;
        self.build(l, m, p * 2);
        self.build(m + 1, r, p * 2 + 1);
        self.pushup(p);
    }

    fn range_query(&mut self, l: usize, r: usize, cl: usize, cr: usize, p: usize) -> T {
        if l <= cl && cr <= r {
            return self.tree[p];
        }
        self.pushdown(cl, cr, p);
        let m = cl + (cr - cl) / 2;
        let mut res: T = T::default();
        if l <= m {
            res += self.range_query(l, r, cl, m, p * 2);
        }
        if m < r {
            res += self.range_query(l, r, m + 1, cr, p * 2 + 1);
        }
        res
    }

    fn range_add(&mut self, l: usize, r: usize, cl: usize, cr: usize, p: usize, val: T) {
        if l <= cl && cr <= r {
            self.tree[p] += val * T::from(cr - cl + 1);
            self.lazy[p] += val;
            return;
        }
        self.pushdown(cl, cr, p);
        let m = cl + (cr - cl) / 2;
        if l <= m {
            self.range_add(l, r, cl, m, p * 2, val);
        }
        if m < r {
            self.range_add(l, r, m + 1, cr, p * 2 + 1, val);
        }
        self.pushup(p);
    }

    pub fn query(&mut self, l: usize, r: usize) -> T {
        self.range_query(l, r, 1, self.n - 1, 1)
    }

    pub fn add(&mut self, l: usize, r: usize, val: T) {
        self.range_add(l, r, 1, self.n - 1, 1, val)
    }
}

fn main() {
    let nums: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut seg = SegTree::new(&nums);
    println!("{}", seg.query(1, 8));
    seg.add(1, 8, 10);
    println!("{}", seg.query(1, 8));
}
