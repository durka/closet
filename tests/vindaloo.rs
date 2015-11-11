#[macro_use] extern crate closet;

#[test]
fn adds() {
    let add = vindaloo!(|x, y| x + y);

    assert_eq!(
        vec![1, 2, 3].into_iter()
                     .map(add(2))
                     .collect::<Vec<_>>(),
        vec![3, 4, 5]
    );
    assert_eq!(
        vec![1, 2, 3].into_iter()
                     .map(add(3))
                     .collect::<Vec<_>>(),
        vec![4, 5, 6]
    );
}

#[test]
fn types() {
    let apply = vindaloo!(|f: Box<Fn(i32, i32) -> i32>, x: i32, y: i32| -> i32 { f(x, y) });

    assert_eq!(
        vec![1, 2, 3].into_iter()
                     .map(apply(Box::new(|x, y| x + y))
                               (2))
                     .collect::<Vec<_>>(),
        vec![3, 4, 5]
    );
    assert_eq!(
        vec![1, 2, 3].into_iter()
                     .map(apply(Box::new(|x, y| x * y))
                               (2))
                     .collect::<Vec<_>>(),
        vec![2, 4, 6]
    );
}

