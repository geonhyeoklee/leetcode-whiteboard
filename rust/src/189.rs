struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            if let Some(num) = nums.pop() {
                nums.reverse();
                nums.push(num);
                nums.reverse();
            }
        }
    }
}

fn main() {
    let mut nums = vec![1, 2, 3];
    let k = 4;

    Solution::rotate(&mut nums, k);
}
