#[macro_use]
extern crate count;

#[test]
fn count_empty() {
    let count = count!();
    assert_eq!(count, 0);
}

#[test]
fn count_one() {
    let count = count!(10);
    assert_eq!(count, 1);

    let count = count!(20,);
    assert_eq!(count, 1);
}

#[test]
fn count_multiple() {
    let count = count!(10, 100, 21, 3, "a");
    assert_eq!(count, 5);

    let count = count!(10, 21, 3, "a",);
    assert_eq!(count, 4);
}

#[test]
fn vec_empty() {
    let vec: Vec<()> = my_vec![];

    assert!(vec.is_empty());
    assert_eq!(vec, vec![]);
}

#[test]
fn vec_list() {
    let vec = my_vec!["a", "v", "ddd"];
    let expected = vec!["a", "v", "ddd"];

    assert_eq!(vec.len(), 3);
    assert_eq!(vec.capacity(), 3);
    assert_eq!(vec, expected);

    let vec = my_vec![1, 2, 3, 4, 5,];
    let expected = vec![1, 2, 3, 4, 5];

    assert_eq!(vec.len(), 5);
    assert_eq!(vec.capacity(), 5);
    assert_eq!(vec, expected);
}

#[test]
fn vec_clone() {
    let vec = my_vec!["a"; 10];
    let expected = my_vec!["a"; 10];

    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);
    assert_eq!(vec, expected);

    #[derive(Debug, Clone, PartialEq)]
    struct A(i32);

    let vec = my_vec![A(0); 20];
    let expected = vec![A(0); 20];

    assert_eq!(vec.len(), 20);
    assert_eq!(vec.capacity(), 20);
    assert_eq!(vec, expected);
}

#[test]
fn vec_literal() {
    let vec = my_vec![10 => "a"];
    let expected = vec!["a"; 10];

    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);
    assert_eq!(vec, expected);

    #[derive(Debug, PartialEq)]
    struct A(i32);

    let vec = my_vec![20 => A(0)];

    assert_eq!(vec.len(), 20);
    assert_eq!(vec.capacity(), 20);
    assert_eq!(vec, (0..20).map(|_| A(0)).collect::<Vec<_>>());
}
