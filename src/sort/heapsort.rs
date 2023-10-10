pub fn heapfiy<T>(nums: &mut Vec<T>, start: usize, end: usize)
where
    T: PartialOrd,
{
    let mut far = start;
    let mut child = far * 2 + 1;
    while child <= end {
        if child < end && nums[child] < nums[child + 1] {
            child += 1;
        }
        if nums[child] < nums[far] {
            break;
        }
        nums.swap(far, child);
        far = child;
        child = far * 2 + 1;
    }
}

pub fn heapsort<T>(nums: &mut Vec<T>)
where
    T: PartialOrd,
{
    for i in (0..=(nums.len() / 2 - 1)).rev() {
        heapfiy(nums, i, nums.len() - 1);
    }

    for i in (1..=(nums.len() - 1)).rev() {
        nums.swap(0, i);
        heapfiy(nums, 0, i - 1);
    }
}

#[cfg(test)]
mod test {
    use super::heapsort;
    #[test]
    fn test1() {
        let mut nums = vec![3, 1, 2, 5, 6];
        heapsort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 5, 6]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![217, 219, 1, 23, 232];
        heapsort(&mut nums);
        assert_eq!(nums, [1, 23, 217, 219, 232]);
    }
}
