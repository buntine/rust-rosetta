type Node<T> = Option<Box<Link<T>>>;

struct Link<T> {
    value: T,
    next: Node<T>,
}

struct LinkedList<T> {
    head: Option<Link<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList{head: None}
    }

    pub fn insert(&mut self, value: T) -> LinkedList<T> {
        let h = match self.head.take() {
            Some(n) => Some(Link{value: value, next: Some(Box::new(n))}),
            None => None,
        };

        LinkedList{head: h}
    }
}

#[test]
fn it_works() {
    let mut ll: LinkedList<i32> = LinkedList::new();
    ll.insert(1);
    ll.insert(2);
}
