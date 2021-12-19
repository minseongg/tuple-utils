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
