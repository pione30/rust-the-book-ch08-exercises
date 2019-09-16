mod exercise1;
mod exercise2;
mod exercise3;

use std::collections::HashMap;

fn main() {
    let mut test_vec: Vec<usize> = vec![1, 2, 3, 5, 2, 9, 42];
    let answer1 = exercise1::calc_mean_median_mode(&mut test_vec);
    assert_eq!(answer1.mean, 9);
    assert_eq!(answer1.median, 3);
    assert_eq!(answer1.mode, 2);

    assert_eq!(exercise2::to_pig_latin(&"first".to_string()), Some("irst-fay".to_string()));
    assert_eq!(exercise2::to_pig_latin(&"apple".to_string()), Some("apple-hay".to_string()));
    assert_eq!(exercise2::to_pig_latin(&"Grüße, Jürgen ❤".to_string()), None);
    assert_eq!(exercise2::to_pig_latin(&"s".to_string()), None);
    assert_eq!(exercise2::to_pig_latin(&"".to_string()), None);

    let mut payroll = HashMap::new();
    let payroll = exercise3::add_an_employee(&mut payroll, "Add Sally to Engineering".to_string());
    let mut payroll = payroll.ok().unwrap();
    assert_eq!(payroll["Engineering"], vec!["Sally"]);

    let payroll = exercise3::add_an_employee(&mut payroll, "Add Amir to Sales".to_string());
    let mut payroll = payroll.ok().unwrap();
    assert_eq!(payroll["Sales"], vec!["Amir"]);

    let payroll = exercise3::add_an_employee(&mut payroll, "Add John to Engineering".to_string());
    let payroll = payroll.ok().unwrap();
    assert_eq!(payroll["Engineering"], vec!["John", "Sally"]);
}
