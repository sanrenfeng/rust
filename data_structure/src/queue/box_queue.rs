use super::Queue;

struct BoxQueue<T> {
    front : Option<Box<QueueNode<T>>>,
}

struct QueueNode<T> {
    data : T,
    next : Option<Box<QueueNode<T>>>,
}

impl<T> Queue<T> for BoxQueue<T> {
    fn new() -> Self {
        Self {
            front : None,
        }
    }
    fn is_empty(&self) -> bool {
        self.front.is_none()
    }
    fn len(&self) -> usize {
        let mut front = &self.front;
        let mut len = 0 ;
        while let Some(current) = front {
            len += 1;
            front = &(*current).next;
        }
        len
    }
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let mut pre;
        let mut cur = &self.front;
        while let Some(node) = cur {
            pre =  
            pre = node;
            next  = &node.next;
        }
        None 
    }

    fn push(&mut self, ele: T) {
        let front = QueueNode {
            data : ele,
            next : self.front,
        };
        self.front = Some(Box::new(front));
    }
}
