fn dicho(e: i32, array: &[i32]) -> bool {
    let mut i = 0;
    let mut j = array.len() - 1;
    while i < j {
        let m = (i + j) / 2;
        if array[m] < e {
            i = (m + 1) as usize;
        } else {
            j = m as usize;
        }
    };
    return array[j] == e;
}
fn main() {
    let a = [0, 1, 4, 7, 8, 11];
    let b = [-5, 2, 3, 7, 9];
    assert_eq!(dicho(5, &a), false);
    assert_eq!(dicho(1, &a), true);
    assert_eq!(dicho(0, &a), true);
    assert_eq!(dicho(8, &b), false);
    assert_eq!(dicho(9, &a), true);
}
