use super::Queue;

struct VecQueue<T> {
    data : Vec<T>,
}

impl<T> Queue<T> for VecQueue<T> {
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn new() -> Self {
        Self { data: vec![] }
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    fn push(&mut self, ele: T) {
        self.data.insert(0,ele);
    }
}

#[test]
fn test_len() {
    let mut queue : VecQueue<&str> = Queue::new();
    queue.push("a");
    queue.push("b");
    queue.push("c");
    queue.pop();
    assert_eq!(2,queue.len());
}

#[test]
fn test_push_pop() {
    let mut queue : VecQueue<&str> = Queue::new();
    queue.push("a");
    queue.push("b");
    queue.push("c");
    queue.pop();
    queue.push("d");
    assert_eq!("b",queue.pop().unwrap());
}

#[test]
fn test_is_empty() {
    let mut queue : VecQueue<&str> = Queue::new();
    queue.push("a");
    queue.push("b");
    queue.push("c");
    queue.pop();
    queue.pop();
    queue.pop();

    assert_eq!(true,queue.is_empty());
}