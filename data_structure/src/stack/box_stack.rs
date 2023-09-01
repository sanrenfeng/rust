use super::Stack;

#[derive(Debug)]
struct BoxStack<T> {
    top: Option<Box<StackNode<T>>>,
}

#[derive(Debug)]
struct StackNode<T> {
    data: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> Stack<T> for BoxStack<T> {
    fn is_empty(&self) -> bool {
        self.top.is_none()
    }
    fn len(&self) -> usize {
        let mut stack_node = &self.top;
        let mut len = 0;
        while let Some(node) = stack_node {
            len += 1;
            stack_node = &node.next;
        }
        len
    }
    fn new() -> Self {
        Self { top: None }
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        } else {
            let top = self.top.take().unwrap();
            self.top = top.next;
            return Some(top.data);
        }
    }

    fn push(&mut self,ele: T) {
        let top = StackNode {data: ele,next : self.top.take()};
        self.top = Some(Box::new(top));
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

    let mut s: BoxStack<&TestStruct> = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(), None);

    s.push(&a);
    s.push(&b);
    println!("{:?}", s);

    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);
}
