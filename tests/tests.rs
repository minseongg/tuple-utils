use tuple_utils::*;

#[derive(Debug, PartialEq)]
struct Foo<T>(T);

#[derive(Debug, PartialEq)]
struct Bar<T>(T);

#[test]
fn test_arity() {
    assert_eq!(<() as Arity>::arity(), 0);
    assert_eq!(<(Foo<usize>, Bar<String>) as Arity>::arity(), 2);
    assert_eq!(<(f32, f32, f32, f32) as Arity>::arity(), 4);
}

#[test]
fn test_split_first() {
    assert_eq!((3, 5).split_first(), (3, (5,)));
    assert_eq!((Foo(1), Bar(2), "hoho").split_first(), (Foo(1), (Bar(2), "hoho")));
}

#[test]
fn test_split_last() {
    assert_eq!((3, 5).split_last(), ((3,), 5));
    assert_eq!((Foo(1), Bar(2), "hoho").split_last(), ((Foo(1), Bar(2)), "hoho"));
}

#[test]
fn test_push_front() {
    assert_eq!((2,).push_front(1), (1, 2));
    assert_eq!((Bar(2), "hoho").push_front(Foo(1)), (Foo(1), Bar(2), "hoho"));
}

#[test]
fn test_push_back() {
    assert_eq!((1,).push_back(2), (1, 2));
    assert_eq!((Foo(1), Bar(2)).push_back("hoho"), (Foo(1), Bar(2), "hoho"));
}
