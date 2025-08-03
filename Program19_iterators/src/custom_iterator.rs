struct Counter {
    count: u32,
}
impl Counter {
    //constructor. each time first value will be 0
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32; //iterams are unsigned int
    //only method req to impl
    fn next(&mut self) -> Option<Self::Item> {
        if self.count <= 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
#[test]
fn calling_next_directly() {
    let mut counter: Counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), Some(6));
    assert_eq!(counter.next(), None);
}
