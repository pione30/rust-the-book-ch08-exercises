use std::collections::HashMap;

#[derive(Debug)]
pub struct MeanMedianMode {
    pub mean: usize,
    pub median: usize,
    pub mode: usize,
}

pub fn calc_mean_median_mode(vec_int: &mut Vec<usize>) -> MeanMedianMode {
    // destructive sort
    vec_int.sort();

    let len = vec_int.len();
    let sum = vec_int.iter().fold(0, |sum, val| sum + val);

    let mean = sum / len;

    let median = match vec_int.get(len / 2) {
        Some(val) => *val,
        None => unreachable!(),
    };

    let mut count_map = HashMap::new();

    for val in vec_int.iter() {
        let count = count_map.entry(*val).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut tmp_count = std::usize::MIN;

    for (vec_val, count) in count_map.iter() {
        if *count > tmp_count {
            tmp_count = *count;
            mode = *vec_val
        };
    }

    MeanMedianMode {
        mean: mean,
        median: median,
        mode: mode,
    }
}
