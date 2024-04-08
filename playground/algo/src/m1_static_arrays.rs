/*
    static arrays are represented as contiguous elements in RAM
    static arrays are fixed sized
    w/r i-th element is O(1)
    inserting/deleting last element is O(1)
    inserting/deleting (by shifting) elemnt in the middle is O(n)
*/

#[cfg(test)]
mod test {

    #[test]
    fn test_static_arrays() {
        let mut arr = [1, 2, 3, 4, 5]; // Fixed size
        assert_eq!(1, arr[0]); // Big O(1)
        arr[0] = 11; // Big O(1)
        assert_eq!(11, arr[0]);
        assert_eq!(5, arr.len());
    }
}
