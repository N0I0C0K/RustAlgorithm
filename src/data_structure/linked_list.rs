use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

type Node<T> = Rc<RefCell<LinkedListNode<T>>>;

#[derive(Debug, PartialEq, Eq)]
struct LinkedListNode<T> {
    val: Option<T>,
    next: Option<Node<T>>,
    pre: Option<Node<T>>,
}

impl<T> LinkedListNode<T> {
    pub fn new(val: Option<T>) -> Node<T> {
        Rc::new(RefCell::new(LinkedListNode {
            val: val,
            next: None,
            pre: None,
        }))
    }
}

struct LinkedList<T> {
    head: Node<T>,
    tail: Node<T>,
    size: usize,
}

impl<T> LinkedList<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        let head = LinkedListNode::new(None);
        let tail = LinkedListNode::new(None);
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().pre = Some(head.clone());
        LinkedList {
            head: head,
            tail: tail,
            size: 0,
        }
    }

    pub fn insert_back(&mut self, pos: &Node<T>, val: T) -> Node<T> {
        let new_node = LinkedListNode::new(Some(val));
        let mut new_mut = new_node.borrow_mut();
        new_mut.next = pos.borrow().next.clone();
        new_mut.pre = Some(pos.clone());

        pos.borrow().next.as_ref().and_then(|p| {
            p.borrow_mut().pre = Some(new_node.clone());
            None::<T>
        });
        pos.borrow_mut().next = Some(new_node.clone());

        new_node.clone()
    }

    pub fn insert_front(&mut self, pos: &Node<T>, val: T) -> Node<T> {
        let new_node = LinkedListNode::new(Some(val));
        let mut new_mut = new_node.borrow_mut();
        new_mut.next = Some(pos.clone());
        new_mut.pre = pos.borrow().pre.clone();

        pos.borrow().pre.as_ref().and_then(|p| {
            p.borrow_mut().next = Some(new_node.clone());
            None::<T>
        });
        pos.borrow_mut().pre = Some(new_node.clone());

        new_node.clone()
    }

    pub fn earse(&mut self, pos: &Node<T>) {}

    pub fn push_back(&mut self, val: T) -> Node<T> {
        let pos = self.tail.borrow().pre.clone().unwrap();
        self.size += 1;
        self.insert_back(&pos, val)
    }

    pub fn push_front(&mut self, val: T) -> Node<T> {
        let pos = self.head.clone();
        self.size += 1;
        self.insert_back(&pos, val)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let head = LinkedListNode::new(Some(1));

        let tail = LinkedListNode::new(Some(2));
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().next = Some(head.clone());

        assert_eq!(
            Rc::ptr_eq(&head.borrow().next.as_ref().unwrap(), &tail),
            true
        );
    }
}
