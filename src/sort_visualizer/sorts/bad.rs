use crate::sort_visualizer::structures::{Sort, SortList};

pub struct StoogeSort;

impl StoogeSort {
    fn sort_internal(list: &mut SortList, start: usize, end: usize) {
        if end < start {
            return;
        }

        list.swap_if_more(start, end);

        if end - start >= 2 {
            let third = (end - start + 1) / 3;
            Self::sort_internal(list, start, end - third);
            Self::sort_internal(list, start + third, end);
            Self::sort_internal(list, start, end - third);
        }
    }
}

impl Sort for StoogeSort {
    fn sort(list: &mut SortList) {
        Self::sort_internal(list, 0, list.len() - 1);
    }
}

pub struct SlowSort;

impl SlowSort {
    fn sort_internal(list: &mut SortList, start: usize, end: usize) {
        if end <= start {
            return;
        }

        let middle = (end + start) / 2;
        Self::sort_internal(list, start, middle);
        Self::sort_internal(list, middle + 1, end);

        list.swap_if_more(start, end);

        Self::sort_internal(list, start, end - 1);
    }
}

impl Sort for SlowSort {
    fn sort(list: &mut SortList) {
        Self::sort_internal(list, 0, list.len() - 1);
    }
}
