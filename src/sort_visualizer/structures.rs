mod auxs;
mod sort_list;
mod sort_op;

pub use auxs::AuxHandle;
pub use sort_list::SortList;
pub use sort_op::SortOp;

pub trait Sort {
    fn sort(list: &mut SortList);
}
