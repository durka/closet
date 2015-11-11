#[macro_use] extern crate closet;

use std::sync::Arc;

#[derive(Debug)]
struct S;

impl Clone for S {
    fn clone(&self) -> Self {
        println!("cloning!");
        S
    }
}

#[test]
fn none() {
    assert_eq!(
        clone_army!([] move | | 42)(),
        42
    );
}

#[test]
fn one() {
    let a = Arc::new(41);
    assert_eq!(
        clone_army!([a] move |b| format!("{}", &*a + b))(1),
        String::from("42")
    );
}

#[test]
fn two() {
    let a = Arc::new(42);
    let b = S;
    assert_eq!(
        clone_army!([a, b] move || format!("{} {:?}", a, b))(),
        String::from("42 S")
    )
}

#[test]
fn types() {
    let a: Arc<i32> = Arc::new(41);
    let b: S = S;
    assert_eq!(
        clone_army!([a, b] move |c: Arc<i32>, d: S| format!("{} {:?} {:?}", &*a + &*c, b, d))(Arc::new(1), S),
        String::from("42 S S")
    );
}

