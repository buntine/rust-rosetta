type Node<T> = Option<Box<Link<T>>>;

struct Link<T> {
    value: T,
    next: Node<T>,
}

struct LinkedList<T> {
    head: Node<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList{head: None}
    }

    pub fn insert(&mut self, value: T) {
        let h = self.head.take();
        self.head = Some(Box::new(Link{value: value, next: h}))
    }

    fn lookup<'a>(&'a self, base: &'a Node<T>, offset: usize) -> Option<&T> {
        match *base {
            Some(ref h) if offset == 0 => Some(&h.value),
            Some(ref h) => self.lookup(&h.next, offset - 1),
            _ => None,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.lookup(&self.head, index)
    }
}

#[test]
fn it_works() {
    let mut ll: LinkedList<i32> = LinkedList::new();
    assert_eq!(ll.get(0), None);
}

#[test]
fn insertion() {
    let mut ll: LinkedList<i32> = LinkedList::new();
    assert_eq!(ll.get(0), None);

    ll.insert(1);
    assert_eq!(ll.get(0), Some(&1));

    ll.insert(2);
    assert_eq!(ll.get(0), Some(&2));
}

#[test]
fn lookup() {
    let mut ll: LinkedList<i32> = LinkedList::new();
    ll.insert(3);
    ll.insert(2);
    ll.insert(1);

    assert_eq!(ll.get(0), Some(&1));
    assert_eq!(ll.get(1), Some(&2));
    assert_eq!(ll.get(2), Some(&3));
    assert_eq!(ll.get(3), None);
}
