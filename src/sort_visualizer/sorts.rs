mod bad;
mod comparison;
mod counting;
mod networks;
mod simple;

use crate::sort_visualizer::structures::{Sort, SortList};
use bad::*;
use comparison::*;
use counting::*;
use simple::*;

macro_rules! sorts {
    ($amt: literal, $($variant:ident $struct_name: ident $display_name: literal, )*) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum Sorts {
            $($variant ,)*
        }

        impl Sorts {
            pub const VARIANTS: [Self; $amt] = [$(Self::$variant),* ];

            pub fn index_of(&self) -> usize {
                Self::VARIANTS.iter().position(|v| v == self).unwrap()
            }

            pub const fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant => $display_name,)*
                }
            }

            pub fn apply_sort(&self, list: &mut SortList) {
                match self {
                    $(Self::$variant => $struct_name::sort(list),)*
                }
            }
        }
    };
}

sorts! {
    16,
    Bubble BubbleSort "Bubble Sort",
    Comb CombSort "Comb Sort",
    Insertion InsertionSort "Insertion Sort",
    Shell ShellSort "Shell Sort",
    Selection SelectionSort "Selection Sort",
    Icbic ICantBelieveItCanSort "I Can't Believe It Can Sort",
    Quick QuickSort "Quick Sort",
    RecursiveMerge RecursiveMergeSort "Recursive Merge Sort",
    IterativeMerge IterativeMergeSort "Iterative Merge Sort",
    Heap HeapSort "Heap Sort",
    Smooth SmoothSort "Smooth Sort",
    Counting CountingSort "Counting Sort",
    LSDRadix LSDRadixSort "LSD Radix Sort",
    RecursiveMSDRadix RecursiveMSDRadixSort "Recursive MSD Radix Sort",
    Stooge StoogeSort "Stooge Sort",
    Slow SlowSort "Slow Sort",
}
