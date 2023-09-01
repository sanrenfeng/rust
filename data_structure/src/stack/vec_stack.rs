#[allow(unused)]
use std::vec;

use super::Stack;

// 栈数据结构
#[derive(Debug)]
struct Vec_Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> for Vec_Stack<T> {
    fn new() -> Self {
        Self { data: vec![] }
    }
    fn push(&mut self, ele: T) {
        self.data.push(ele);
    }
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}

#[test]
fn test() {
    #[derive(PartialEq, Eq, Debug)]
    struct TestStruct {
        a: i32,
    }

    let a = TestStruct { a: 5 };
    let b = TestStruct { a: 9 };

    let mut s: Vec_Stack<&TestStruct> = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(), None);

    s.push(&a);
    s.push(&b);
    println!("{:?}", s);

    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);
}
