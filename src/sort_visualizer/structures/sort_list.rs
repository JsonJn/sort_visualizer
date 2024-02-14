use crate::sort_visualizer::structures::auxs::{Aux, AuxHandle};
use crate::sort_visualizer::structures::SortOp;
use rand::prelude::SliceRandom;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct SortList {
    data: Vec<u32>,
    ops: Vec<SortOp>,

    min_ident: u32,
    auxs: HashMap<AuxHandle, Aux>,
    last_create_ops: Vec<usize>,
}

impl SortList {
    pub(crate) fn debug(&self) {
        println!("{:?}", self.data)
    }
}

impl SortList {
    pub(super) fn add_op(&mut self, sort_op: SortOp) {
        self.ops.push(sort_op)
    }

    pub fn get_aux(&self, handle: AuxHandle) -> &Aux {
        &self.auxs[&handle]
    }

    pub fn get_aux_mut(&mut self, handle: AuxHandle) -> &mut Aux {
        self.auxs.get_mut(&handle).unwrap()
    }

    pub fn copy_from(&mut self, aux: AuxHandle, from: usize, to: usize) {
        self.add_op(SortOp::CopyFromAux(aux, from, to));
        let at = self.get_aux(aux)[from];
        self.data[to] = at;
    }

    pub fn push_aux_to_aux(&mut self, from: AuxHandle, from_ind: usize, to: AuxHandle) {
        self.add_op(SortOp::CopyAuxToAux(
            from,
            from_ind,
            to,
            self.get_aux(to).len(),
        ));
        let at = self.get_aux(from)[from_ind];
        self.get_aux_mut(to).push(at);
    }

    pub fn create_auxiliary(&mut self) -> AuxHandle {
        let handle = AuxHandle(self.min_ident);
        self.min_ident += 1;

        self.last_create_ops.push(self.ops.len());
        self.add_op(SortOp::CreateAux(handle, 0));
        self.auxs.insert(handle, Aux::new());
        handle
    }

    pub fn remove_auxiliary(&mut self, handle: AuxHandle) {
        let last_remaining = self.last_create_ops.pop().unwrap();
        let len = self.get_aux(handle).len();
        if let SortOp::CreateAux(_, size) = &mut self.ops[last_remaining] {
            *size = len;
        }
        self.add_op(SortOp::DestroyAux(handle));

        self.auxs.remove(&handle);
    }

    pub fn create_list(length: usize) -> Vec<u32> {
        let mut list = Vec::new();
        for x in 1..=length {
            list.push(x as u32);
        }
        list
    }

    pub fn create_rand_list(length: usize) -> Vec<u32> {
        let mut list = Self::create_list(length);
        list.shuffle(&mut rand::thread_rng());
        list
    }

    pub fn new(list: Vec<u32>) -> Self {
        Self {
            data: list,
            ops: Vec::new(),
            auxs: HashMap::new(),
            last_create_ops: Vec::new(),
            min_ident: 0,
        }
    }

    pub fn take_ops(self) -> Vec<SortOp> {
        self.ops
    }

    pub fn unstored_read(&mut self, addr: usize) -> u32 {
        self.data[addr]
    }

    pub fn read(&mut self, addr: usize) -> u32 {
        self.add_op(SortOp::Read(addr));
        self.data[addr]
    }

    pub fn compare(&mut self, a: usize, b: usize) -> Ordering {
        self.add_op(SortOp::Compare(a, b));
        self.data[a].cmp(&self.data[b])
    }

    pub fn write(&mut self, addr: usize, val: u32) {
        self.add_op(SortOp::Write(addr, val));
        self.data[addr] = val;
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.add_op(SortOp::Swap(a, b));
        self.data.swap(a, b);
    }

    pub fn swap_if_more(&mut self, a: usize, b: usize) {
        self.add_op(SortOp::SwapIfMore(a, b));
        if self.data[a] > self.data[b] {
            self.data.swap(a, b);
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn last(&self) -> usize {
        self.len() - 1
    }
}
