pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    *nums = nums
        .iter()
        .filter(|&n| *n != val)
        .cloned()
        .collect::<Vec<i32>>();
    nums.len() as i32
}

#[cfg(test)]
mod test {
    use super::remove_element;

    #[test]
    fn test_remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        let k = remove_element(&mut nums, 3);
        assert_eq!(k, 2);
        // dbg!("nums= {} - k= {}", nums, k);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let k = remove_element(&mut nums, 2);
        assert_eq!(k, 5);
        // dbg!("nums= {} - k= {}", nums, k);
    }
}
