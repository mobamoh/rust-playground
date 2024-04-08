/*
    dynamic arrays are represented as contiguous elements in RAM
    dynamic arrays can grow in size
    w/r i-th element is O(1)
    pushing/popping last element is O(1)
    inserting/removing (by shifting) elemnt in the middle is O(n)
*/

#[cfg(test)]
mod test {

    #[test]
    fn test_dynamic_array() {
        let mut dyn_arr = vec![6, 7, 8];
        dyn_arr.push(4); // pushing is Big O(1) 
        assert_eq!(dyn_arr[3], 4); // [6,7,8,4]

        dyn_arr.insert(0, 5); // inserting is Big O(n)
        assert_eq!(dyn_arr[0], 5); // [5,6,7,8,4]

        dyn_arr.remove(1); // removing is Big O(n)
        assert_eq!(dyn_arr[1], 7); // [5,7,8,4]
        dbg!("{}",dyn_arr.len());
    }
}
