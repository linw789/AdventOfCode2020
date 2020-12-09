use std::cmp::PartialOrd;

pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    let mut last_swapped_pos = v.len() - 1;
    loop {
        let mut swapped = false;
        for i in 0..last_swapped_pos {
            if v[i] > v[i+1] {
                v.swap(i, i+1);
                last_swapped_pos = i;
                swapped = true;
            }
        }
        if swapped == false {
            break;
        }
    }
}

pub fn insert_sort<T: PartialOrd>(v: &mut [T]) {
    if v.len() == 1 {
        return;
    }

    for i in 1..v.len() {
        let mut j = i;
        while j != 0 {
            if v[j] < v[j-1] {
                v.swap(j - 1, j);
            } else {
                break;
            }
            j -= 1;
        }
    }
}
