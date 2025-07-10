enum Message {
    Sample1,
    Sapmple2 { x: i32, y: i32 },
    Sample3Add(i32, i32),
}
impl Message {
    fn calling(&self) {
        match self {
            Self::Sample1 => {
                println!("Sample1");
            }
            Message::Sapmple2 { x, y } => {
                println!("{}+{}={}", x, y, x + y)
            }
            Message::Sample3Add(x, y) => {
                println!("{}+{}={}", x, y, x + y)
            }
        }
    }
}
pub fn main() {
    println!("------------------------------------------------------------------------");
    let a = Message::Sample1;
    let b = Message::Sapmple2 { x: 32, y: 32 };
    let c = Message::Sample3Add(30, 32);
    a.calling();
    b.calling();
    c.calling();
}
