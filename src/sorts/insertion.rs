pub fn sort(a: &mut [i32]) -> &mut [i32] {
    for i in 1..a.len() {
        let mut j = i;
        while j > 0 && a[j-1] > a[j] {
            let tmp = a[j];
            a[j] = a[j-1];
            a[j-1] = tmp;
            j-=1;
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_test() {
        let mut a: [i32; 5] = [3, 4, 1, 5, 2];
        let a = sort(&mut a[..]);
        assert_eq!(a, [1, 2, 3, 4, 5]);
    }
}