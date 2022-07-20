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
}
