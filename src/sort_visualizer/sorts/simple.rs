use crate::sort_visualizer::structures::{Sort, SortList};
use std::cmp::Ordering;

pub struct BubbleSort;
impl Sort for BubbleSort {
    fn sort(list: &mut SortList) {
        for x in (1..=list.len()).rev() {
            let mut done = true;
            for j in 1..x {
                let i = j - 1;
                if list.compare(i, j) == Ordering::Greater {
                    list.swap(i, j);
                    done = false;
                }
            }
            if done {
                break;
            }
        }
    }
}

pub struct CombSort;

const SHRINK_FACTOR: f32 = 1.0 / 1.3;

impl Sort for CombSort {
    fn sort(list: &mut SortList) {
        let mut gap_size = list.len();
        loop {
            gap_size = ((gap_size as f32) * SHRINK_FACTOR) as usize;
            if gap_size < 1 {
                gap_size = 1;
            } else if gap_size == 9 || gap_size == 10 {
                gap_size = 11;
            }

            let mut done = true;
            for j in gap_size..list.len() {
                let i = j - gap_size;
                if list.compare(i, j) == Ordering::Greater {
                    list.swap(i, j);
                    done = false;
                }
            }
            if gap_size == 1 && done {
                break;
            }
        }
    }
}

pub struct InsertionSort;
impl Sort for InsertionSort {
    fn sort(list: &mut SortList) {
        for to_insert in 1..list.len() {
            let mut at = to_insert;
            while at > 0 && list.compare(at - 1, at) == Ordering::Greater {
                list.swap(at - 1, at);
                at -= 1;
            }
        }
    }
}

pub struct SelectionSort;

impl Sort for SelectionSort {
    fn sort(list: &mut SortList) {
        for x in 0..list.len() {
            let mut min_index = x;
            for i in (x + 1)..list.len() {
                if list.compare(min_index, i) == Ordering::Greater {
                    min_index = i;
                }
            }
            list.swap(min_index, x);
        }
    }
}

pub struct ShellSort;

impl ShellSort {
    const GAPS: [usize; 8] = [701, 301, 132, 57, 23, 10, 4, 1];
}

impl Sort for ShellSort {
    fn sort(list: &mut SortList) {
        for gap in Self::GAPS {
            if gap >= list.len() {
                continue;
            }
            for to_insert in gap..list.len() {
                let mut at = to_insert;
                while at >= gap && list.compare(at - gap, at) == Ordering::Greater {
                    list.swap(at - gap, at);
                    at -= gap;
                }
            }
        }
    }
}

pub struct ICantBelieveItCanSort;

impl Sort for ICantBelieveItCanSort {
    fn sort(list: &mut SortList) {
        for x in 0..list.len() {
            for y in 0..list.len() {
                list.swap_if_more(y, x);
            }
        }
    }
}
