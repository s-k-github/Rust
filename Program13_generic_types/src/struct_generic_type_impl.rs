struct Point<T> {
    x: T,
    y: T,
}
impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

//below provide x and y both functions where both are float
impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}
pub fn struct_generic_type_with_impl() {
    let int_struct = Point { x: 20, y: 34 };
    int_struct.x();
    // int_struct.y(); //error coz no such method implemented for i32
    let float_struct = Point { x: 20.9, y: 34.8 };
    float_struct.x();
    float_struct.y();
}
//////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Point1<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point1<T, U> {
    fn mix<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}
pub fn struct_with_different_types() {
    let p1 = Point1 { x: 20.9, y: 34 };
    let p2 = Point1 { x: "Hello", y: 'w' };
    let p3 = p1.mix(p2);
    println!("{:?}", p3)
}
