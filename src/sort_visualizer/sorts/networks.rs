use crate::sort_visualizer::structures::{Sort, SortList};

pub struct BitonicSort;

impl BitonicSort {
    fn cmp_swap(list: &mut SortList, a: usize, b: usize) {
        // print!("({a}, {b}), ");
        // stdout().flush().unwrap();
        if a < list.len() && b < list.len() {
            list.swap_if_more(a, b);
        }
    }
}

impl Sort for BitonicSort {
    fn sort(list: &mut SortList) {
        let full_size = (list.len() as f32).log2().ceil() as usize;
        for size in 0..full_size {
            let half_width = 1 << size;
            let full_width = half_width << 1;
            for index in 0..(list.len() / full_width) {
                let offset = index * full_width;
                for x in 0..half_width {
                    Self::cmp_swap(list, offset + x, offset + full_width - x - 1);
                }
            }

            for size1 in (0..size).rev() {
                let half_width = 1 << size1;
                let full_width = half_width << 1;
                for index in 0..(list.len() / full_width) {
                    let offset = index * full_width;
                    for x in 0..half_width {
                        Self::cmp_swap(list, offset + x, offset + half_width + x);
                    }
                }
            }
        }
    }
}
