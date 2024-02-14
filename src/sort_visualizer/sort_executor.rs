use crate::sort_visualizer::sorts::Sorts;
use crate::sort_visualizer::structures::{AuxHandle, SortList, SortOp};
use std::collections::HashMap;

pub struct SortExecutor {
    data: Vec<u32>,
    auxs: HashMap<AuxHandle, Vec<u32>>,
    sort_ops: Vec<SortOp>,
    index: usize,

    highlighted: Vec<usize>,
    highlighted_aux: HashMap<AuxHandle, Vec<usize>>,
}

impl SortExecutor {
    pub fn new(data: Vec<u32>) -> Self {
        Self {
            data,
            auxs: HashMap::new(),
            sort_ops: Vec::new(),
            index: 0,
            highlighted: Vec::new(),
            highlighted_aux: HashMap::new(),
        }
    }

    pub fn out_of_ops(&self) -> bool {
        self.index >= self.sort_ops.len()
    }

    pub fn next_op(&self) -> Option<&SortOp> {
        self.sort_ops.get(self.index)
    }

    pub fn get_data(&self) -> &Vec<u32> {
        &self.data
    }

    pub fn get_sort_ops(&self) -> &Vec<SortOp> {
        &self.sort_ops
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn set_data(&mut self, data: Vec<u32>) {
        self.data = data;
    }

    pub fn sort_and_replace(&mut self, sort: Sorts) {
        self.sort_ops.clear();
        self.index = 0;
        let mut sort_list = SortList::new(self.data.clone());
        sort.apply_sort(&mut sort_list);
        self.sort_ops = sort_list.take_ops();
    }

    pub fn execute_one(&mut self) {
        if let Some(&op) = self.sort_ops.get(self.index) {
            self.index += 1;
            op.apply(&mut self.data, &mut self.auxs);
            let (h_main, h_aux) = op.accesses();
            self.highlighted.extend(h_main);
            for (handle, index) in h_aux {
                self.highlighted_aux.entry(handle).or_default().push(index);
            }
        }
    }

    pub fn take_and_clear_highlights(&mut self) -> (Vec<usize>, HashMap<AuxHandle, Vec<usize>>) {
        let result = (self.highlighted.clone(), self.highlighted_aux.clone());
        self.highlighted.clear();
        self.highlighted_aux.clear();
        result
    }
}
