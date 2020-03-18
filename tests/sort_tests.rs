use sorts;
use rand::prelude::*;

macro_rules! time {
    ($x:expr, $n:expr) => {{
        (0..$n).fold(std::time::Duration::new(0, 0), |acc, _| {
            let now = std::time::Instant::now();
            $x;
            acc + now.elapsed()
        }) / $n
    }};
}
fn assert_sorted_ascending<T: PartialEq + PartialOrd>(arr: &[T]) {
    for a in arr.windows(2) {
        assert!(a[0] <= a[1]);
    }
}
#[test]
fn check_insertion() {
    let mut test_vec: Vec<usize> = (0..100).rev().collect();
    sorts::insertion_sort(&mut test_vec);
    assert_sorted_ascending(&test_vec);
}
// #[test]
// fn check_odd_even() {
//     let mut rng = rand::thread_rng();
//     let mut test_vec: Vec<usize> =  (0..100_000).map(|_| rng.gen::<usize>()).collect();
//     println!("odd even worstish: {:?}", time!(sorts::odd_even_sort(&mut test_vec), 1));
//     assert_sorted_ascending(&test_vec);
// }
// it works

#[test]
fn bubble() {
    let mut rng = rand::thread_rng();
    let mut test_vec: Vec<usize> =  (0..100_000).map(|_| rng.gen::<usize>()).collect();
    println!("bubble worstish: {:?}", time!(sorts::bubble_sort(&mut test_vec), 1));
    assert_sorted_ascending(&test_vec);
}