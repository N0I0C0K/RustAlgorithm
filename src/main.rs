struct SegTree<'a> {
    n: usize,
    nums: &'a Vec<i32>,
    tree: Vec<i32>,
    lazy: Vec<i32>,
}

impl<'a> SegTree<'a> {
    fn new(nums: &'a Vec<i32>) -> Self {
        let n = nums.len();
        let mut obj = SegTree {
            n,
            nums,
            lazy: vec![0; (n + 1) * 4],
            tree: vec![0; (n + 1) * 4],
        };
        obj.build(1, n - 1, 1);
        obj
    }

    fn pushup(&mut self, p: usize) {
        self.tree[p] = self.tree[p * 2] + self.tree[p * 2 + 1];
    }

    fn pushdown(&mut self, l: usize, r: usize, p: usize) {
        if l == r || self.lazy[p] == 0 {
            return;
        }
        let tmp = self.lazy[p];
        let m = l + (r - l) / 2;
        self.tree[p * 2] += tmp * ((m - l + 1) as i32);
        self.tree[p * 2 + 1] += tmp * ((r - m) as i32);

        self.lazy[p] = 0;
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

    fn range_query(&mut self, l: usize, r: usize, cl: usize, cr: usize, p: usize) -> i32 {
        if l <= cl && cr <= r {
            return self.tree[p];
        }
        self.pushdown(cl, cr, p);
        let m = cl + (cr - cl) / 2;
        let mut res = 0;
        if l <= m {
            res += self.range_query(l, r, cl, m, p * 2);
        }
        if m < r {
            res += self.range_query(l, r, m + 1, cr, p * 2 + 1);
        }
        res
    }

    fn range_add(&mut self, l: usize, r: usize, cl: usize, cr: usize, p: usize, val: i32) {
        if l <= cl && cr <= r {
            self.tree[p] += val * ((cr - cl + 1) as i32);
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

    pub fn query(&mut self, l: usize, r: usize) -> i32 {
        self.range_query(l, r, 1, self.n - 1, 1)
    }

    pub fn add(&mut self, l: usize, r: usize, val: i32) {
        self.range_add(l, r, 1, self.n - 1, 1, val)
    }
}

fn main() {
    let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut seg = SegTree::new(&nums);
    println!("{}", seg.query(1, 8));
    seg.add(1, 8, 10);
    println!("{}", seg.query(1, 8));
}
