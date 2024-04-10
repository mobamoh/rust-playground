/*
https://leetcode.com/problems/concatenation-of-array/description/
 */
pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    [nums.as_slice(), nums.as_slice()].concat()

    // let len = nums.len();
    // let mut ans: Vec<i32> = vec![0; len * 2];
    // for i in 0..len {
    //     ans[i] = nums[i];
    //     ans[i + len] = nums[i];
    // }

    // ans
    // let len = nums.len();
    // let mut ans: Vec<i32> = vec![0; len * 2];
    // println!("{:?}", ans);
    // let mut i = 0;
    // loop {
    //     ans[i] = nums[i];
    //     ans[i + len] = nums[i];
    //     i += 1;
    //     if i == len {
    //         break;
    //     }
    // }

    // ans
}

#[cfg(test)]
mod test {
    use super::get_concatenation;

    #[test]
    fn test_get_concatenation() {
        let nums = vec![1, 2, 1];
        let ans = get_concatenation(nums);
        dbg!(ans);
    }
}
