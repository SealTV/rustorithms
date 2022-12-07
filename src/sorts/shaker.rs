#[allow(dead_code)]
pub fn shaker_sort(a: &mut [i32]) -> &mut [i32] {
    let mut b = true;

    let mut beg = 0;
    let mut end = a.len() - 1;

    while b {
        b = false;

        for i in beg..end {
            if a[i] > a[i + 1] {
                let tmp = a[i];
                a[i] = a[i + 1];
                a[i + 1] = tmp;

                b = true;
            }
        }

        beg += 1;

        if !b {
            break;
        }

        let mut i = end;
        loop {
            if i <= beg {
                break;
            }

            if a[i] < a[i - 1] {
                let tmp = a[i];
                a[i] = a[i - 1];
                a[i - 1] = tmp;

                b = true;
            }

            i -= 1;
        }

        end -= 1;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shaker_sort_test() {
        let mut a: [i32; 5] = [5, 1, 3, 4, 2];
        let a = shaker_sort(&mut a[..]);
        assert_eq!(a, [1, 2, 3, 4, 5])
    }
}
