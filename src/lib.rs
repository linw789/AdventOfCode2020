use std::cmp::PartialOrd;

pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    let mut last_swapped_pos = v.len() - 1;
    loop {
        let mut swapped = false;
        for i in 0..last_swapped_pos {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
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
            if v[j] < v[j - 1] {
                v.swap(j - 1, j);
            } else {
                break;
            }
            j -= 1;
        }
    }
}

pub fn quick_sort<T: PartialOrd + Copy>(v: &mut [T]) {
    fn partition<T: PartialOrd + Copy>(v: &mut [T]) -> usize {
        let pivot = *v.last().unwrap();
        // The goal is to move all elements that are smaller than `pivot` to the left of the
        // elements that are greater than or equal to `pivot`. To do so, we check each element and
        // swap it with the current left-most 'gte' element if it's smaller than `pivot`, then
        // update the 'gte' element index. `gte_i` tracks the left-most of 'gte' elements.
        let mut gte_i = 0;
        for i in 0..(v.len() - 1) {
            if v[i] < pivot {
                if i != gte_i {
                    v.swap(i, gte_i);
                }
                gte_i += 1;
            }
        }
        if gte_i != (v.len() - 1) {
            v.swap(gte_i, v.len() - 1);
        }

        return gte_i;
    }

    if v.len() > 1 {
        let split = partition(v);
        quick_sort(&mut v[0..split]);
        quick_sort(&mut v[split..]);
    }
}
