mod expull;
mod memory_arena;
mod term_hashmap;

pub use crate::expull::ExpUnrolledLinkedList;
pub use crate::memory_arena::{Addr, MemoryArena};
pub use crate::term_hashmap::{compute_table_size, TermHashMap};

pub type UnorderedTermId = u32;

#[inline(always)]
fn pop_2_bytes(arr: &[u8]) -> [u8; 2] {
    [arr[0], arr[1]]
}
