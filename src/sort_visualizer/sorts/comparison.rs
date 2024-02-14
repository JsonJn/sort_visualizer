use crate::sort_visualizer::structures::{Sort, SortList};
use std::cell::RefCell;
use std::cmp::Ordering;

pub struct QuickSort;
impl QuickSort {
    fn sort_internal(list: &mut SortList, from: usize, to: usize) {
        if from == to {
            return;
        }
        // if to - from < 128 {
        //     const GAPS: [usize; 5] = [57, 23, 10, 4, 1];
        //     for gap in GAPS {
        //         for to_insert in from..=to {
        //             let mut at = to_insert;
        //             while at >= from + gap && list.compare(at - gap, at) == Ordering::Greater {
        //                 list.swap(at - gap, at);
        //                 at -= gap;
        //             }
        //         }
        //     }
        //     return;
        // }
        let mut start = from;
        let mut end = to;
        while end != start {
            let b1 = list.compare(start, to) == Ordering::Less;
            let b2 = list.compare(end, to) == Ordering::Less;
            match (b1, b2) {
                (true, true) => start += 1,
                (true, false) => {
                    start += 1;
                    // end -= 1;
                }
                (false, true) => {
                    list.swap(start, end);
                    start += 1;
                    // end -= 1;
                }
                (false, false) => end -= 1,
            }
        }
        list.swap(end, to);
        if start > from {
            Self::sort_internal(list, from, start - 1);
        }
        if end < to {
            Self::sort_internal(list, end + 1, to);
        }
    }
}
impl Sort for QuickSort {
    fn sort(list: &mut SortList) {
        // Self::sort_internal(list, 0, list.len() - 1);
        let mut stack = vec![(0, list.len() - 1)];
        while let Some((from, to)) = stack.pop() {
            if from == to {
                return;
            }
            let mut start = from;
            let mut end = to;
            while end != start {
                let b1 = list.compare(start, to) == Ordering::Less;
                let b2 = list.compare(end, to) == Ordering::Less;
                match (b1, b2) {
                    (true, true) => start += 1,
                    (true, false) => {
                        start += 1;
                        // end -= 1;
                    }
                    (false, true) => {
                        list.swap(start, end);
                        start += 1;
                        // end -= 1;
                    }
                    (false, false) => end -= 1,
                }
            }
            list.swap(end, to);
            if start > from {
                stack.push((from, start - 1));
            }
            if end < to {
                stack.push((end + 1, to));
            }
        }
    }
}

fn mergesort_merge(list: &mut SortList, start: usize, middle: usize, end: usize) {
    let len = end - start + 1;
    let merged = list.create_auxiliary();
    let (mut p1, mut p2) = (start, middle + 1);
    for _ in 0..len {
        if p2 > end {
            merged.push_from(list, p1);
            p1 += 1;
        } else if p1 > middle {
            merged.push_from(list, p2);
            p2 += 1;
        } else if list.compare(p1, p2) != Ordering::Greater {
            merged.push_from(list, p1);
            p1 += 1;
        } else {
            merged.push_from(list, p2);
            p2 += 1;
        }
    }
    for i in 0..merged.len(list) {
        list.copy_from(merged, i, start + i);
    }
    list.remove_auxiliary(merged);
}

pub struct RecursiveMergeSort;

impl RecursiveMergeSort {
    fn sort_internal(list: &mut SortList, start: usize, end: usize) {
        if start >= end {
            return;
        }
        let middle = (end + start) / 2;
        Self::sort_internal(list, start, middle);
        Self::sort_internal(list, middle + 1, end);

        mergesort_merge(list, start, middle, end);
    }
}

impl Sort for RecursiveMergeSort {
    fn sort(list: &mut SortList) {
        Self::sort_internal(list, 0, list.len() - 1)
    }
}

pub struct IterativeMergeSort;

impl Sort for IterativeMergeSort {
    fn sort(list: &mut SortList) {
        let mut size = 2;
        loop {
            let half_size = size / 2;
            for i in 0..(list.len() / size + 1) {
                let start = size * i;
                let middle = start + half_size - 1;
                if middle >= list.len() {
                    continue;
                }
                let end = start + size - 1;
                if end >= list.len() {
                    mergesort_merge(list, start, middle, list.len() - 1);
                } else {
                    mergesort_merge(list, start, middle, end);
                }
            }
            if size >= list.len() {
                return;
            }
            size *= 2;
        }
    }
}

pub struct HeapSort;

impl HeapSort {
    fn sift_down(list: &mut SortList, node: usize, end: usize) {
        let (c1, c2) = (2 * node + 1, 2 * node + 2);
        #[allow(clippy::if_same_then_else)]
        let greater_child = if c1 > end && c2 > end {
            return;
        } else if c1 > end {
            c2
        } else if c2 > end {
            c1
        } else if list.compare(c1, c2) == Ordering::Greater {
            c1
        } else {
            c2
        };

        if list.compare(greater_child, node) == Ordering::Greater {
            list.swap(greater_child, node);
            Self::sift_down(list, greater_child, end);
        }
    }
}

impl Sort for HeapSort {
    fn sort(list: &mut SortList) {
        for node in (0..=(list.last() / 2)).rev() {
            let end = list.len() - 1;
            Self::sift_down(list, node, end);
        }

        for end in (1..=list.last()).rev() {
            let root = 0;
            list.swap(root, end);
            Self::sift_down(list, 0, end - 1);
        }
    }
}

thread_local! {
    static LEONARDO_CACHE: RefCell<Vec<usize>> = RefCell::new(vec![1, 1]);
}

pub struct SmoothSort;

impl SmoothSort {
    fn leonardo_extend_one() -> (usize, usize) {
        let new_index = LEONARDO_CACHE.with_borrow(|v| v.len());
        let (a, b) = LEONARDO_CACHE.with_borrow(|l| (l[new_index - 1], l[new_index - 2]));
        let new = a + b + 1;
        LEONARDO_CACHE.with_borrow_mut(|l| l.push(new));
        (new_index, new)
    }

    fn leonardo(n: usize) -> usize {
        if let Some(cached) = LEONARDO_CACHE.with_borrow(|v| v.get(n).copied()) {
            cached
        } else {
            let length = LEONARDO_CACHE.with_borrow(|v| v.len());
            let mut at = length;
            loop {
                let (_, extended) = Self::leonardo_extend_one();
                if at == n {
                    break extended;
                }
                at += 1;
            }
        }
    }

    fn children(root: usize, leo_size: usize) -> (usize, usize) {
        let right_size = Self::leonardo(leo_size - 2);
        let right_root = root - 1;
        let left_root = right_root - right_size;
        (left_root, right_root)
    }

    fn sift_down_top(list: &mut SortList, stretches: &[usize], root: usize, stretch_index: usize) {
        let stretch_leo_size = stretches[stretch_index];
        if stretch_index == 0 {
            Self::sift_down(list, root, stretch_leo_size);
        } else if stretch_leo_size > 1 {
            let stepson_root = root - Self::leonardo(stretch_leo_size);
            let (left_root, right_root) = Self::children(root, stretch_leo_size);

            let (max_child, max_child_leo_size) =
                if list.compare(left_root, right_root) == Ordering::Greater {
                    (left_root, stretch_leo_size - 1)
                } else {
                    (right_root, stretch_leo_size - 2)
                };

            if list.compare(stepson_root, max_child) == Ordering::Greater {
                if list.compare(stepson_root, root) == Ordering::Greater {
                    list.swap(stepson_root, root);
                    Self::sift_down_top(list, stretches, stepson_root, stretch_index - 1);
                }
            } else if list.compare(max_child, root) == Ordering::Greater {
                list.swap(max_child, root);
                Self::sift_down(list, max_child, max_child_leo_size);
            }
        } else {
            let stepson_root = root - 1;
            if list.compare(stepson_root, root) == Ordering::Greater {
                list.swap(stepson_root, root);
                Self::sift_down_top(list, stretches, stepson_root, stretch_index - 1);
            }
        }
    }

    fn sift_down(list: &mut SortList, root: usize, leo_size: usize) {
        if leo_size > 1 {
            let (left_root, right_root) = Self::children(root, leo_size);

            let (max_child, max_child_leo_size) =
                if list.compare(left_root, right_root) == Ordering::Greater {
                    (left_root, leo_size - 1)
                } else {
                    (right_root, leo_size - 2)
                };

            if list.compare(max_child, root) == Ordering::Greater {
                list.swap(max_child, root);
                Self::sift_down(list, max_child, max_child_leo_size);
            }
        }
    }
}

impl Sort for SmoothSort {
    fn sort(list: &mut SortList) {
        let mut stretch_leo_size: Vec<usize> = vec![1, 1];
        for to_add in 2..list.len() {
            // println!("{:?}", stretch_leo_size);
            // Self::check_heap_property(list, &stretch_leo_size);
            let l = stretch_leo_size.len();
            if l >= 2 {
                let left_size = stretch_leo_size[l - 2];
                let right_size = stretch_leo_size[l - 1];
                if (left_size == 1 && right_size == 1) || left_size.saturating_sub(right_size) == 1
                {
                    let new_size = left_size + 1;
                    stretch_leo_size.pop();
                    stretch_leo_size.pop();
                    stretch_leo_size.push(new_size);

                    // println!("merging...");
                    Self::sift_down_top(list, &stretch_leo_size, to_add, l - 2);
                } else {
                    stretch_leo_size.push(1);
                    Self::sift_down_top(list, &stretch_leo_size, to_add, l);
                }
            } else {
                stretch_leo_size.push(1);
                Self::sift_down_top(list, &stretch_leo_size, to_add, l);
            }
        }

        for to_remove in (2..list.len()).rev() {
            let l = stretch_leo_size.len();
            let leo_size = stretch_leo_size[l - 1];
            stretch_leo_size.pop();
            if leo_size > 1 {
                stretch_leo_size.push(leo_size - 1);
                stretch_leo_size.push(leo_size - 2);
                let left_root = to_remove - Self::leonardo(leo_size - 2) - 1;
                Self::sift_down_top(list, &stretch_leo_size, left_root, l - 1);
                Self::sift_down_top(list, &stretch_leo_size, to_remove - 1, l);
            }
        }
    }
}
