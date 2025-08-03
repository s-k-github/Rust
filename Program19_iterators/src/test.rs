pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
#[test]
fn iterator_test() {
    let v1: Vec<i32> = vec![4, 5, 6];
    let mut v1_iter: std::vec::IntoIter<i32> = v1.into_iter();
    assert_eq!(v1_iter.next(), Some(4));
    assert_eq!(v1_iter.next(), Some(5));
    assert_eq!(v1_iter.next(), Some(6));
    assert_eq!(v1_iter.next(), None);
}
