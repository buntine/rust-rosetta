type Node<T> = Option<Box<Link<T>>>;

struct Link<T> {
    value: T,
    next: Node<T>,
}

struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new(value: T) -> LinkedList<T> {
        LinkedList{head: Link{value: value, next: None}}
    }

    pub fn insert(&self, value: T) -> LinkedList<T> {
        let h = self.head;
        LinkedList{head: Link{value: value, next: Some(Box::new(h))}}
    }
}

#[test]
fn it_works() {
    let ll = LinkedList::new(1);
}
