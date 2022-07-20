pub fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    if nums.len() <= 1 {
        return nums.len();
    }

    let mut pos: usize = 0;
    for i in 1..nums.len() {
        if nums[pos] != nums[i] {
            pos += 1;
            nums[pos] = nums[i];
        }
    }
    return pos + 1;
}

pub fn remove_duplicates2(nums: &mut Vec<i32>) -> usize {
    if nums.len() <= 2 {
        return nums.len();
    }

    let mut pos: usize = 1;
    for i in 2..nums.len() {
        if nums[pos - 1] != nums[i] {
            pos += 1;
            nums[pos] = nums[i];
        }
    }
    return pos + 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_duplicates_test() {
        let mut nums = vec![0, 1, 1, 2, 3, 4, 4, 8];
        let count = remove_duplicates(&mut nums);
        assert_eq!(count, 6);
    }

    #[test]
    fn remove_duplicates2_test() {
        let mut nums = vec![1, 1, 2, 2, 2, 3, 4, 5, 5, 5, 5, 6, 7, 7, 8, 8, 8];
        let count = remove_duplicates2(&mut nums);
        assert_eq!(count, 13);
    }
}
