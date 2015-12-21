struct Person<'a> {
    name: &'a str,
    parent: Option<&'a Person<'a>>,
}

impl<'a> Person<'a> {
    fn new(name: &'a str, parent: Option<&'a Person<'a>>) -> Person<'a> {
        Person{name: name, parent: parent}
    }
}

#[test]
fn it_works() {
    let jane = Person::new("Jane", None);
    let tom = Person::new("Tom", Some(&jane));

    assert_eq!(tom.name, "Tom");
    assert_eq!(jane.name, "Jane");
    assert_eq!(tom.parent.unwrap().name, "Jane");
}
