struct PriorityQueue<T> {
    que: Vec<T>,
    cmp: Box<dyn Fn(&T, &T) -> bool>,
}

impl<T> PriorityQueue<T>
where
    T: PartialOrd + Clone,
{
    pub fn heapify(&mut self, start: usize, end: usize) {
        let mut far = start;
        let mut child = far * 2 + 1;
        while child <= end {
            if child < end && (self.cmp)(&self.que[child + 1], &self.que[child]) {
                child += 1;
            }

            if (self.cmp)(&self.que[far], &self.que[child]) {
                break;
            }
            self.que.swap(far, child);
            far = child;
            child = far * 2 + 1;
        }
    }

    pub fn new<F>(nums: Option<Vec<T>>, cmp: F) -> Self
    where
        F: Fn(&T, &T) -> bool + 'static,
    {
        let cmp_func: Box<dyn Fn(&T, &T) -> bool> = Box::new(cmp);

        if let Some(nums) = nums {
            let mut r = PriorityQueue {
                que: nums,
                cmp: cmp_func,
            };
            for i in (0..=r.que.len() / 2 - 1).rev() {
                r.heapify(i, r.que.len() - 1);
            }
            return r;
        }

        PriorityQueue {
            que: vec![],
            cmp: cmp_func,
        }
    }

    pub fn len(&self) -> usize {
        self.que.len()
    }

    pub fn empty(&self) -> bool {
        self.que.is_empty()
    }

    pub fn pop(&mut self) -> T {
        if self.empty() {
            panic!("queue is empty");
        }
        let mut far = 0;
        let mut child = far * 2 + 1;
        while child < self.len() {
            if child < self.len() - 1 && (self.cmp)(&self.que[child + 1], &self.que[child]) {
                child += 1;
            }
            self.que.swap(far, child);

            far = child;
            child = far * 2 + 1;
        }
        let l = self.len();
        self.que.swap(far, l - 1);
        self.que.pop().unwrap()
    }

    pub fn push(&mut self, item: T) {
        self.que.push(item);
        let l = self.len();
        let mut far = l / 2 - 1;
        while far >= 0 {
            let mut child = far * 2 + 1;
            if child < l - 1 && (self.cmp)(&self.que[child + 1], &self.que[child]) {
                child += 1;
            }
            if (self.cmp)(&self.que[far], &self.que[child]) {
                break;
            }
            self.que.swap(far, child);
            if far == 0 {
                break;
            }
            far = (far - 1) / 2;
        }
    }
}

#[cfg(test)]
mod test {
    use super::PriorityQueue;
    #[test]
    fn test_min() {
        let nums = vec![4, 3, 1, 2];
        let mut que = PriorityQueue::new(Some(nums), |x, y| x < y);
        assert_eq!(que.pop(), 1);
        que.push(-2);
        assert_eq!(que.pop(), -2);
        assert_eq!(que.pop(), 2);
        assert_eq!(que.pop(), 3);
    }

    #[test]
    fn test_max() {
        let nums = vec![4, 3, 1, 2];
        let mut que = PriorityQueue::new(Some(nums), |x, y| x > y);
        assert_eq!(que.pop(), 4);
        que.push(100);
        assert_eq!(que.pop(), 100);
        assert_eq!(que.pop(), 3);
        assert_eq!(que.pop(), 2);
    }
}
