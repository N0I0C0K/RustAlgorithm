struct LinkedListNode<T> {
    val: Option<T>,
    next: Option<Box<LinkedListNode<T>>>,
    pre: Option<Box<LinkedListNode<T>>>,
}

struct LinkedList<T> {
    head: LinkedListNode<T>,
    tail: LinkedListNode<T>,
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
