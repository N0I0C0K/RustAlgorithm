pub fn partition<T>(nums: &mut Vec<T>, start: usize, end: usize) -> usize
where
    T: PartialOrd,
{
    let mut i = start;

    for j in start..end {
        if &nums[j] > &nums[end] {
            nums.swap(j, i);
            i += 1;
        }
    }
    nums.swap(i, end);
    i
}

pub fn _topk<T>(nums: &mut Vec<T>, k: usize, start: usize, end: usize)
where
    T: PartialOrd,
{
    if start >= end {
        return;
    }
    let t = partition(nums, start, end);
    if t > k - 1 {
        _topk(nums, k, start, t - 1);
    } else if t < k - 1 {
        _topk(nums, k, t + 1, end);
    } else {
        return;
    }
}

pub fn topk<T>(mut nums: Vec<T>, k: usize) -> Vec<T>
where
    T: PartialOrd,
{
    let l = nums.len();
    _topk(&mut nums, k, 0, l - 1);

    nums.into_iter().take(k).collect()
}

#[cfg(test)]
mod test {
    use super::topk;
    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(topk(nums, 1), [5]);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3, 4, 5];
        let mut res = topk(nums, 3);
        res.sort();
        assert_eq!(res, [3, 4, 5]);
    }
}
