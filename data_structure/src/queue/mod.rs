pub mod vec_queue;
pub mod box_queue;

pub trait Queue<T> {
    fn new() -> Self;
    fn push(&mut self, ele: T);
    fn pop(&mut self) -> Option<T>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
