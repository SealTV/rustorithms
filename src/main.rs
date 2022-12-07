mod sorts;
use sorts::bubble;

fn main() {
    let mut a:[i32;5] = [4, 5, 3, 1, 2];
    let a = bubble::bubble_sort(&mut a[..]);
    // let a = 
    assert_eq!(a, [1, 2, 3, 4, 5]);
}
