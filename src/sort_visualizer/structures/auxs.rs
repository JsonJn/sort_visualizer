use crate::sort_visualizer::structures::{SortList, SortOp};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct AuxHandle(pub u32);

impl AuxHandle {
    pub fn len(&self, list: &mut SortList) -> usize {
        list.get_aux(*self).len()
    }

    pub fn push_from(&self, list: &mut SortList, from: usize) {
        let len = list.get_aux(*self).len();
        list.add_op(SortOp::CopyToAux(from, *self, len));
        let at = list.unstored_read(from);
        list.get_aux_mut(*self).push(at)
    }

    pub fn unstored_push_from(&self, list: &mut SortList, from: usize) {
        let at = list.unstored_read(from);
        list.get_aux_mut(*self).push(at)
    }
}

pub struct Aux {
    data: Vec<u32>,
    max_length: usize,
}

impl Aux {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            max_length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, value: u32) {
        self.data.push(value);
        self.max_length = self.max_length.max(self.data.len());
    }
}

impl Index<usize> for Aux {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Aux {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
