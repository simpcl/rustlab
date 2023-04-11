use std::vec::Vec;

fn bubble_sort(nums: &mut Vec<i32>) {
    for i in 0..nums.len() {
        let max = nums.len() - i;
        for j in 1..max {
            if nums[j - 1] > nums[j] {
                let n = nums[j - 1];
                nums[j - 1] = nums[j];
                nums[j] = n;
            }
        }
    }
}

fn partition_nums(nums: &mut Vec<i32>, begin: usize, end: usize) -> usize {
    let mut low = begin;
    let mut high = end;
    let v = nums[low];
    while low < high {
        while low < high {
            if v < nums[high] {
                high -= 1;
            } else {
                nums[low] = nums[high];
                break;
            }
        }
        while low < high {
            if v > nums[low] {
                low += 1;
            } else {
                nums[high] = nums[low];
                break;
            }
        }
    }
    nums[low] = v;
    low
}

fn quick_sort_part(nums: &mut Vec<i32>, begin: usize, end: usize) {
    if begin >= end {
        return;
    }
    let mid = partition_nums(nums, begin, end);
    quick_sort_part(nums, begin, mid - 1);
    quick_sort_part(nums, mid + 1, end);
}

fn quick_sort(nums: &mut Vec<i32>) {
    quick_sort_part(nums, 0, nums.len() - 1);
}

fn display<T: std::fmt::Display>(v: &Vec<T>) {
    for n in v {
        println!("{}", n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut v = vec![1, 0, 3, 2, 5, 7, 8, 6];
        bubble_sort(&mut v);
        for n in &v {
            println!("{}", n);
        }
    }

    #[test]
    fn quick_sort_test() {
        let mut v = vec![1, 0, 3, 2, 5, 7, 8, 6];
        quick_sort(&mut v);
        display(&v);
    }
}
