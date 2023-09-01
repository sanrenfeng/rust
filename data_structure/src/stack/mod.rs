pub mod vec_stack;
pub mod box_stack;

pub trait Stack<T> {
    fn new() -> Self;
    fn push(&mut self,ele: T);
    fn pop(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}
