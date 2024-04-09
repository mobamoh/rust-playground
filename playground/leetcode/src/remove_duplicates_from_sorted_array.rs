/*
   https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
*/
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 1;
    let mut last_unel = nums[0];
    for i in k..nums.len() {
        if nums[i] == last_unel {
            continue;
        }
        nums[k] = nums[i]; //[0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        last_unel = nums[i];
        k += 1;
    }
    k as i32

    // let mut i = 0;
    // let mut j = 1;
    // loop {
    //     if j >= nums.len() {
    //         break;
    //     }

    //     if nums[i] != nums[j] {
    //         i += 1;
    //         nums[i] = nums[j];
    //     }

    //     j += 1;
    // }
    // i += 1;
    // i as i32

    // let mut tmp = Vec::new();
    // for num in nums.clone() { // Big O(n)
    //     if !tmp.contains(&num) {
    //         tmp.push(num);
    //     }
    // }
    // let len = tmp.len() as i32;
    // *nums = tmp;
    // len
}

#[cfg(test)]
mod test {
    use super::remove_duplicates;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = remove_duplicates(&mut nums);
        dbg!("{}", k);
        assert_eq!(k, 5);
        dbg!("{}", nums);

        let mut nums = vec![1, 1, 2];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 2);
        dbg!("{}", nums);
    }
}
