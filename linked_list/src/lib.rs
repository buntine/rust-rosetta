type Node<T> = Option<Box<Link<T>>>;

struct Link<T> {
    value: T,
    next: Node<T>,
}

struct LinkedList<T> {
    head: Node<T>,
}

impl<T> LinkedList<T> {
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

    pub fn remove_at(&mut self, index: usize) {
        match index {
            0 => self.remove(),
            i @ _ => {
                // Iterate through to node of i - 1
                // Set node.next = node.next.next in a safe way;
            }
        }
    }

    pub fn insert_at(&mut self, index: usize, value: T) {
        match index {
            0 => self.insert(value),
            i @ _ => {
                // Iterate through to node of i - 1
                // Set node.next = Node(value: value, next: node.next) in a safe way;
            }
        }
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
