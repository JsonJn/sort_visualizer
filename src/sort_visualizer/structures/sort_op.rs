use crate::sort_visualizer::structures::AuxHandle;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum SortOp {
    Read(usize),
    Compare(usize, usize),
    Write(usize, u32),
    Swap(usize, usize),
    SwapIfMore(usize, usize),
    CopyToAux(usize, AuxHandle, usize),
    CopyFromAux(AuxHandle, usize, usize),
    CopyAuxToAux(AuxHandle, usize, AuxHandle, usize),
    CreateAux(AuxHandle, usize),
    DestroyAux(AuxHandle),
}

impl SortOp {
    pub fn apply(&self, list: &mut [u32], auxs: &mut HashMap<AuxHandle, Vec<u32>>) {
        match *self {
            SortOp::Read(_) => {}
            SortOp::Compare(_, _) => {}
            SortOp::Write(a, v) => list[a] = v,
            SortOp::Swap(a, b) => list.swap(a, b),
            SortOp::SwapIfMore(a, b) => {
                if list[a] > list[b] {
                    list.swap(a, b);
                }
            }
            SortOp::CopyToAux(from, h, to) => {
                let at = list[from];
                auxs.get_mut(&h).unwrap()[to] = at;
            }
            SortOp::CopyFromAux(h, from, to) => {
                let at = auxs[&h][from];
                list[to] = at;
            }
            SortOp::CreateAux(handle, size) => {
                auxs.insert(handle, vec![0; size]);
            }
            SortOp::DestroyAux(handle) => {
                auxs.remove(&handle);
            }
            SortOp::CopyAuxToAux(f, fi, t, ti) => {
                let at = auxs[&f][fi];
                auxs.get_mut(&t).unwrap()[ti] = at;
            }
        }
    }

    pub fn accesses(&self) -> (Vec<usize>, Vec<(AuxHandle, usize)>) {
        match *self {
            SortOp::Read(x) => (vec![x], vec![]),
            SortOp::Compare(a, b) => (vec![a, b], vec![]),
            SortOp::Write(a, _) => (vec![a], vec![]),
            SortOp::Swap(a, b) => (vec![a, b], vec![]),
            SortOp::SwapIfMore(a, b) => (vec![a, b], vec![]),
            SortOp::CopyToAux(a, h, b) => (vec![a], vec![(h, b)]),
            SortOp::CopyFromAux(h, a, b) => (vec![b], vec![(h, a)]),
            SortOp::CreateAux(_, _) => (vec![], vec![]),
            SortOp::DestroyAux(_) => (vec![], vec![]),
            SortOp::CopyAuxToAux(h1, a, h2, b) => (vec![], vec![(h1, a), (h2, b)]),
        }
    }
}
