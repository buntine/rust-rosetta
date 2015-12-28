#[derive(PartialEq, Eq, Debug)]
struct Age(u8);

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

// This cannot be elided because Rust doesn't know to use lifetime of "a" or "b".
fn double_first_five<'a>(a: &'a String, b: &'a String) -> (&'a str, &'a str) {
    (&a[..5], &b[..5])
}

// Output borrows generally must have an input value from which it can borrow. The exception is
// the static lifetime.
// Couldn't Rust infer from the body of the function that the borrows are from the static lifetime?
//   - I feel that Rust favours explicitness over terseness. A functions behaviour should be
//   encoded directly into it's signature.
fn output_only<'a>() -> (&'static u8, &'static u8) {
    static X: u8 = 8;
    static Y: u8 = 9;

    (&X, &Y)
}

// Here we are promising Rust that all of the borrowed u8's
// live for the same scope.
// Q: Why do I need a lifetime here?
fn join_ages<'a, T: Iterator>(iters: T) -> Vec<&'a Age>
        where T::Item: IntoIterator<Item=&'a Age> {
    iters.flat_map(|b| b.into_iter())
         .collect()
}

// This is not really necessary here. Both a and b could use the same lifetime
// as Rust only cares that a and b are available for the scope of this function.
// This could also be elided.
fn multi_lifetimes<'a, 'b>(a: &'a u8, b: &'b u8) -> bool {
    (a + b) > 10
}

fn multiple_lifetimes() -> bool {
    let a = &9;

    {
        let b = &9;

        multi_lifetimes(a, b)
    }
}

#[test]
fn it_works() {
    let jane = Person::new("Jane", None);
    let tom = Person::new("Tom", Some(&jane));
    let name = "Andrew".to_owned();
    let another = "Timothy".to_owned();
    let a = 43;
    let vec_ages = vec![vec![Age(90), Age(80)],
                        vec![Age(2)]];

    // Error: This cannot work because the lifetime of the borrow (Rob) is only valid for
    // the expression on the right-hand side. We must increase it's lifetime by giving it a name
    // via 'let'.
    //let bob = Person::new("Bob", Some(&Person::new("Rob", None)));

    // If we swap these two lines around then this will not compile because the lifetime of Mary
    // would be less than that of Bob.
    let mary = Person::new("Mary", None);
    let mut bob = Person::new("Bob", None);

    bob.parent = Some(&mary);

    let arr_ages = vec![[Age(90), Age(80)],
                        [Age(2), Age(45)]];

    assert_eq!(output_only(), (&8, &9));

    assert_eq!(first_five(&name), "Andre");
    assert_eq!(double_first_five(&name, &another), ("Andre", "Timot"));

    assert!(multiple_lifetimes());

    assert_eq!(tom.name, "Tom");
    assert_eq!(jane.name, "Jane");
    assert_eq!(tom.parent.unwrap().name, "Jane");
    assert_eq!(tom.parents_name(), Some("Jane"));
    assert_eq!(jane.parents_name(), None);

    assert_eq!(join_ages(vec_ages.iter()), vec![&Age(90), &Age(80), &Age(2)]);
    assert_eq!(join_ages(arr_ages.iter()), vec![&Age(90), &Age(80), &Age(2), &Age(45)]);
}

// - We are making a promise to the compiler that all of these things live for atleast the same scope.
// - We are not asking the compiler to "create a lifetime". We are just telling it to complain if we
//   don't do what we say we are doing.
// - Nor are you telling Rust to "allow" a binding to be available in a given named scope/lifetime.
// - Why? Because this allows Rust to ensure we cannot cause common memory allocation bugs. It does
//   this at compile time. It's a zero-cost abstraction because there is no runtime penalty.
// - It makes code look "uglier". But this is the wrong mindset for writing systems-level programs.
//   The emphasis here is on safety, not expressiveness.
// - When we say "'a", we are talking about a scope in the calling environment.
// - Question: Why can't Rust ensure/infer/elide the struct scenario? Surely any reference to a Parent
//   must *always* live for the same or a bigger scope???
//   - There is an open RFC for this
//   - It may be complex to implement and/or slow down compilation
//   - Eliding in more cases may prevent newcomer Rust programmers from ever encountering
//     lifetimes, which could have a negative impact in the long run. Pedagogy!
