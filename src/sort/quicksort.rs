use std::cmp::Ord;

fn partition<T: Ord>(nums: &mut Vec<T>, lo: usize, hi: usize) -> usize {
    let pivot = hi;
    let mut i = lo;
    for j in lo..=hi {
        if nums[j] < nums[pivot] {
            nums.swap(i, j);
            i += 1;
        }
    }
    nums.swap(i, pivot);
    i
}

fn _sort<T: Ord>(nums: &mut Vec<T>, lo: usize, hi: usize) {
    if lo < hi {
        let m = partition(nums, lo, hi);
        _sort(nums, lo, m - 1);
        _sort(nums, m + 1, hi);
    }
}

pub fn quick_sort<T: Ord>(nums: &mut Vec<T>) {
    _sort(nums, 0, nums.len() - 1);
}

#[cfg(test)]
mod test {
    use super::quick_sort;
    #[test]
    fn test_sort_1() {
        let mut nums = vec![3, 4, 1, 5, 2];
        quick_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_2() {
        let mut nums = vec![1];
        quick_sort(&mut nums);
        assert_eq!(nums, [1]);
    }
}
