pub fn comb_sort(a: &mut [i32]) -> &mut [i32] {
    let k = 1.2473309;

    
    let mut step = a.len() - 1; 

    while step > 1 {
        let mut i :usize = 0;
        while i + step > a.len() {
            if a[i] > a[i + step] {
                let tmp = a[i];
                a[i] = a[i + step];
                a[i + step] = tmp;
            }

            i+=1;
        }

        step = (step as f64 / k) as usize;
    }

    let mut b = true;
    while b {
        b = false;
        for i in 0..a.len()-1 {
            if a[i] > a[i + 1] {
                let tmp = a[i];
                a[i] = a[i + 1];
                a[i+1] = tmp;
                b = true;
            }
        }
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comb_sort_test() {
        let mut a: [i32; 5] = [5, 1, 3, 4, 2];
        let a = comb_sort(&mut a[..]);
        assert_eq!(a, [1, 2, 3, 4, 5])
    }
}
