struct Person<'a> {
    name: &'a str,
    parent: Option<&'a Person<'a>>,
}

impl<'a> Person<'a> {
    fn new(name: &'a str, parent: Option<&'a Person<'a>>) -> Person<'a> {
        Person{name: name, parent: parent}
    }

    fn parents_name(&self) -> Option<&'a str> {
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
