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

#[test]
fn it_works() {
    let jane = Person::new("Jane", None);
    let tom = Person::new("Tom", Some(&jane));

    assert_eq!(tom.name, "Tom");
    assert_eq!(jane.name, "Jane");
    assert_eq!(tom.parent.unwrap().name, "Jane");
    assert_eq!(tom.parents_name(), Some("Jane"));
    assert_eq!(jane.parents_name(), None);
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
// - 
