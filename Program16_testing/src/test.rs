#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn larger_can_hold_larger() {
        let rect1: Rectangle = Rectangle {
            height: 8,
            width: 8,
        };
        let rect2: Rectangle = Rectangle {
            height: 17,
            width: 17,
        };
        assert!(rect1.can_hold(&rect2));
    }

    // #[test]
    fn larger_can_hold_smaller() {
        let rect1: Rectangle = Rectangle {
            height: 8,
            width: 8,
        };
        let rect3: Rectangle = Rectangle {
            height: 7,
            width: 7,
        };
        assert!(rect1.can_hold(&rect3));
    }
}
