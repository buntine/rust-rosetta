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

    pub fn insert(&mut self, value: T) {
        let h = match self.head.take() {
            Some(n) => Some(Box::new(n)),
            None => None,
        };

        self.head = Some(Link{value: value, next: h})
    }

    pub fn first(&self) -> Option<&T> {
        match self.head {
            Some(ref h) => Some(&h.value),
            None => None,
        }
    }
}

#[test]
fn it_works() {
    let mut ll: LinkedList<i32> = LinkedList::new();
    assert_eq!(ll.first(), None);

    ll.insert(1);
    assert_eq!(ll.first(), Some(&1));

    ll.insert(2);
    assert_eq!(ll.first(), Some(&2));
}
