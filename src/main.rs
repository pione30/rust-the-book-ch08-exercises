mod exercise1;

fn main() {
    let mut test_vec: Vec<usize> = vec![1, 2, 3, 5, 2, 9, 42];
    let answer1 = exercise1::calc_mean_median_mode(&mut test_vec);
    assert_eq!(answer1.mean, 9);
    assert_eq!(answer1.median, 3);
    assert_eq!(answer1.mode, 2);
}
