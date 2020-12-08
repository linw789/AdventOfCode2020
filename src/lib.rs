use std::vec::Vec;

pub fn bubble_sort(v: &mut Vec<i32>) {
    loop {
        let mut swapped = false;
        for i in 0..(v.len() - 1) {
            if v[i] > v[i+1] {
                v.swap(i, i+1);
                swapped = true;
            }
        }
        if swapped == false {
            break;
        }
    }
}
