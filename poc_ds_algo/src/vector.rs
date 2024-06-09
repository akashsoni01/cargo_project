use std::arch::aarch64::int32x2_t;

fn two_sum(v: &mut Vec<i32>, target: i32) -> Vec<i32> {
    // for value in v.iter_mut() {
    //
    // }

    let mut tempV = Vec::new();
    if v.len() > 1 {
        for i in 0..v.len() {
            for j in 1..v.len() {
                if (v[i] + v[j]) == target {
                    tempV.append()
                }
            }
        }
    }

    return  v.into_vec();
}