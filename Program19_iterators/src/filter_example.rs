pub fn main() {
    let vector: Vec<i32> = vec![1, 2, 3, 4];
    let evens: Vec<_> = vector.iter().filter(|&x| x % 2 == 0).collect();
    println!("Evens from {:?} are {:?}", vector, evens)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    fn shoe_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == size).collect()
    }
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoe_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                }
            ]
        )
    } //filters_by_size end
} //mod tests end
