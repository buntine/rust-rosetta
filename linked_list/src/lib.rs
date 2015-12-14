type Node<T> = Option<Box<Link<T>>>;

struct Link<T: PartialEq> {
    value: T,
    next: Node<T>,
}

struct LinkedList<T: PartialEq> {
    head: Node<T>,
}

impl<T: PartialEq> LinkedList<T> {
    fn lookup<'a>(&'a self, base: &'a Node<T>, offset: usize) -> Option<&Box<Link<T>>> {
        match *base {
            Some(ref n) if offset == 0 => Some(n),
            Some(ref n) => self.lookup(&n.next, offset - 1),
            _ => None,
        }
    }

    pub fn new() -> LinkedList<T> {
        LinkedList{head: None}
    }

    pub fn insert(&mut self, value: T) {
        let h = self.head.take();
        self.head = Some(Box::new(Link{value: value, next: h}))
    }

    pub fn remove(&mut self) {
        match self.head.take() {
            Some(h) => { self.head = h.next },
            None => { }
        }
    }

    pub fn remove_after(&mut self, find: T) {
    }

    pub fn insert_after(&mut self, find: T, value: T) {
    }
 
    pub fn get(&self, index: usize) -> Option<&T> {
        match self.lookup(&self.head, index) {
            Some(ref n) => Some(&n.value),
            None => None,
        }
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
fn removal() {
    let mut ll: LinkedList<i32> = LinkedList::new();
    ll.insert(1);
    ll.insert(2);

    assert_eq!(ll.get(0), Some(&2));

    ll.remove();
    assert_eq!(ll.get(0), Some(&1));

    ll.remove();
    assert_eq!(ll.get(0), None);
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
