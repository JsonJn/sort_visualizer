use crate::sort_visualizer::structures::{Sort, SortList};
use std::cmp::Ordering;

fn binary_bucket_sort(list: &mut SortList, from: usize, to: usize, mask: u32) {
    let lesser = list.create_auxiliary();
    let greater = list.create_auxiliary();
    for i in from..to {
        let at = list.read(i);
        if at & mask == 0 {
            lesser.push_from(list, i);
        } else {
            greater.push_from(list, i);
        }
    }
    for i in 0..greater.len(list) {
        list.push_aux_to_aux(greater, i, lesser);
        // lesser.append(&mut greater);
    }
    list.remove_auxiliary(greater);
    for i in (from..to).rev() {
        list.copy_from(lesser, i - from, i);
        // list.write(i, lesser[i - from]);
    }
    list.remove_auxiliary(lesser);
}

pub struct LSDRadixSort;

impl Sort for LSDRadixSort {
    fn sort(list: &mut SortList) {
        let mut max_ind = 0;
        for i in 1..list.len() {
            if list.compare(i, max_ind) == Ordering::Greater {
                max_ind = i;
            }
        }

        let max_val = list.read(max_ind);
        let mut mask = 1;
        while mask <= max_val {
            binary_bucket_sort(list, 0, list.len(), mask);
            mask <<= 1;
        }
    }
}

pub struct RecursiveMSDRadixSort;

impl RecursiveMSDRadixSort {
    fn sort_internal(list: &mut SortList, from: usize, to: usize, mask: u32) {
        if to - from <= 1 {
            return;
        }
        let mut lesser = Vec::new();
        let mut greater = Vec::new();
        for i in from..to {
            let at = list.read(i);
            if at & mask == 0 {
                lesser.push(at);
            } else {
                greater.push(at);
            }
        }
        let l_len = lesser.len();
        lesser.append(&mut greater);
        for i in (from..to).rev() {
            list.write(i, lesser[i - from]);
        }
        Self::sort_internal(list, from, from + l_len, mask >> 1);
        Self::sort_internal(list, from + l_len, to, mask >> 1);
    }
}

impl Sort for RecursiveMSDRadixSort {
    fn sort(list: &mut SortList) {
        let mut max_ind = 0;
        for i in 1..list.len() {
            if list.compare(i, max_ind) == Ordering::Greater {
                max_ind = i;
            }
        }

        let max_val = list.read(max_ind);
        let mask = 2 << max_val.ilog2();
        Self::sort_internal(list, 0, list.len(), mask);
    }
}

pub struct CountingSort;

impl Sort for CountingSort {
    fn sort(list: &mut SortList) {
        let mut max_ind = 0;
        for i in 1..list.len() {
            if list.compare(i, max_ind) == Ordering::Greater {
                max_ind = i;
            }
        }
        let max_val = list.read(max_ind);
        let mut arr = vec![0; max_val as usize + 1];
        for i in (0..list.len()).rev() {
            arr[list.read(i) as usize] += 1;
        }
        let mut index = 0;
        for i in 0..list.len() {
            for _ in 0..arr[i] {
                list.write(index, i as u32);
                index += 1;
            }
        }
    }
}
