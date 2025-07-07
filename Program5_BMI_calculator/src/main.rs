use std::io;
fn main() {
    println!("BMI Calculator");
    println!("Please enter your weight in kilograms(kgs) : ");
    let weight = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("Invalid float ordered");
            return;
        }
    };
    println!("Please enter your height in meters : ");
    let height = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("Invalid float ordered");
            return;
        }
    };
    if height == 0.0 || weight == 0.0 {
        println!("Input can not be 0");
        return;
    }
    let bmi = calculate_bmi(height, weight);
    println!("BMI Is : {:.2}", bmi);

    let category = category_bmi(bmi);
    println!("Your category is : {}", category);
}
fn get_input_as_f64() -> Option<f64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<f64>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
fn calculate_bmi(height: f64, weight: f64) -> f64 {
    weight / (height * height)
}
fn category_bmi(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "Underweight"
    } else if bmi < 24.9 {
        "Normal weight"
    } else if bmi < 29.9 {
        "Overweight"
    } else {
        "Obese"
    }
}
