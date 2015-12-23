struct Person<'a> {
    name: &'static str,
    parent: Option<&'a Person<'a>>,
}

impl<'a> Person<'a> {
    fn new(name: &'static str, parent: Option<&'a Person<'a>>) -> Person<'a> {
        Person{name: name, parent: parent}
    }

    fn parents_name(&self) -> Option<&'static str> {
        match self.parent {
            Some(p) => Some(p.name),
            None => None,
        }
    }
}

// This one can be elided.
fn first_five<'a>(value: &'a String) -> &'a str {
    &value[..5]
}

// Q: Why do I need a lifetime here?
pub fn join_iters<'a, T: Iterator>(vecs: T) -> Vec<u8>
        where T::Item: IntoIterator<Item=&'a u8> {
    vecs.flat_map(|b| b.into_iter())
        .cloned()
        .collect()
}

#[test]
fn it_works() {
    let jane = Person::new("Jane", None);
    let tom = Person::new("Tom", Some(&jane));
    let name = "Andrew".to_owned();

    let ints = vec![vec![78, 43, 1],
                    vec![1, 45, 66, 2],
                    vec![99, 8, 20]];

    assert_eq!(first_five(&name), "Andre");

    assert_eq!(tom.name, "Tom");
    assert_eq!(jane.name, "Jane");
    assert_eq!(tom.parent.unwrap().name, "Jane");
    assert_eq!(tom.parents_name(), Some("Jane"));
    assert_eq!(jane.parents_name(), None);

    assert_eq!(join_iters(ints.iter()), vec![78, 43, 1, 1, 45, 66, 2, 99, 8, 20]);
}

// - We are making a promise to the compiler that all of these things live for atleast the same scope.
// - We are not asking the compiler to "create a lifetime". We are just telling it to complain if we
//   don't do what we say we are doing.
// - Why? Because this allows Rust to ensure we cannot cause common memory allocation bugs. It does
//   this at compile time. It's a zero-cost abstraction because there is no runtime penalty.
// - It makes code look "uglier". But this is the wrong mindset for writing systems-level programs.
//   The emphasis here is on safety, not expressiveness.
// - Question: Why can't Rust ensure/infer/elide this scenario? Surely any reference to a Parent
//   must *always* live for the same or a bigger scope???
//   - There is an open RFC for this
//   - It may be complex to implement and/or slow down compilation
//   - Eliding in more cases may prevent newcomer Rust programmers from ever encountering
//     lifetimes, which could have a negative impact in the long run. Pedagogy!
