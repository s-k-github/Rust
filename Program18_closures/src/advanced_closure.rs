use std::{thread, time::Duration};

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    //take 2 seconds to run
    println!("Calculating slowly......");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn example_of_closure() {
    let example_closure = |x| x;
    let a = example_closure(String::from(
        "This will be type of closure since first passed",
    ));
    // let b=example_closure(3)// will throw error since string is considered as type expected
}

pub fn main() {
    let intensity = 10;
    let random_number = 4;
    generate_workout(intensity, random_number);
}
struct Cacher<T>
where
    T: Fn(u32) -> u32, //Fn is provided by standard library. check basic example for details
{
    calculation: T,     //generic called T
    value: Option<u32>, //optional coz when cacher is initialized its gonna be none and then after calculation we will store in value variable
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    //below is constructor function that takes calculation and value wtih None value as input and create new Cacher
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    //below is method because its a reference to self. args are input parameter.
    fn value(&mut self, args: u32) -> u32 {
        match self.value {
            Some(v) => v,
            //none is there coz first time Cacher's value will be none
            None => {
                //below we are calling calculation closure and passing args as input parameter
                let v = (self.calculation)(args);
                //setting value
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // if intensity < 25 {
    //     println!(
    //         "Today do {} pushups",
    //         simulated_expensive_calculation(intensity)
    //     );
    //     println!(
    //         "Next do {} situps",
    //         simulated_expensive_calculation(intensity)
    //     );
    // } else {
    //     if random_number == 3 {
    //         println!("Take a break today! Stay hydrated!");
    //     } else {
    //         println!(
    //             "Today run for {} minutes",
    //             simulated_expensive_calculation(intensity)
    //         );
    //     }
    // }

    // //below is calling expensive_results too many times hence fix that
    // let expensive_results = simulated_expensive_calculation(intensity);
    // if intensity < 25 {
    //     println!("Today do {} pushups", expensive_results);
    //     println!("Next do {} situps", expensive_results);
    // } else {
    //     if random_number == 3 {
    //         println!("Take a break today! Stay hydrated!");
    //     } else {
    //         println!("Today run for {} minutes", expensive_results);
    //     }
    // }

    //use of closures : it does not store return value its storing closure itself
    //but below we are calling expensive closure multiple times to resolve this we will create struct to hold memorization of closure and result of closure
    //input parameter types
    // let expensive_results = |num| {
    //     //take 2 seconds to run
    //     println!("Calculating slowly......");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // if intensity < 25 {
    //     println!("Today do {} pushups", expensive_results(intensity));
    //     println!("Next do {} situps", expensive_results(intensity));
    // } else {
    //     if random_number == 3 {
    //         println!("Take a break today! Stay hydrated!");
    //     } else {
    //         println!("Today run for {} minutes", expensive_results(intensity));
    //     }
    // }

    //input parameter types
    //making below mut coz we will be calling value method which will mutate out Cacher struct
    //after uncommenting below and running following behaviour will be there
    /*1. expensive_results is called only once
    2. the problem is since its called only once value will return single value n number of times since its getting updated at the start only.
      above coz we check self.value which already exist hence it will return some(v)=>v which is old value.
      to fix this use hasmap instead of single value.
      the keys of the hasmap will be the argument passed to the value and value will be result of calling closure with args.
      then inside the value method we need to look up arg inside the hashmap and if value exist then return value else run expensive calculation and store the result.*/
    //there is one mopre issue is we are fixated on u32 as input, to fix this use generic.
    let mut expensive_results = Cacher::new(|num| {
        //take 2 seconds to run
        println!("Calculating slowly......");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today do {} pushups", expensive_results.value(intensity));
        println!("Next do {} situps", expensive_results.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Stay hydrated!");
        } else {
            println!(
                "Today run for {} minutes",
                expensive_results.value(intensity)
            );
        }
    }
}
