#[cfg(test)]
mod bubble_tests {
    use super::super::{*};

    #[test]
    fn bubble_sort_test() -> Result<(), String> {
        let mut a: [i32; 5] = [4, 5, 3, 1, 2];
        let a = bubble_sort(&mut a[..]);
        assert_eq!(a, [1, 2, 3, 4, 5]);
        Ok(())
    }
}
