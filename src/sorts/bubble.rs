#[cfg(test)]
#[path ="./bubble_tests.rs"]
mod bubble_tests;

pub fn sort (a: &mut [i32]) -> &[i32] {
    let mut n = a.len();
    loop {
        let mut swapped = false;
        
        for i in 1..n{
            if a[i-1] > a[i] {
                let k = a[i];
                a[i] = a[i-1];
                a[i-1] = k;
                
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        n = n - 1;
    }
    a
}
