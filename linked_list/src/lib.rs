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
}

#[test]
fn it_works() {
    let ll = LinkedList::new();
}
