
// BMI FORMULA 
// weight(kgs) / height2(m2)
// BMI < 18.5 UW UNDER WEIGHT
// BMI 18.5 - 24.9 N NORMAL 
// BMI 25 - 29.9 OW OVER WEIGHT 
// BMI >= 30 O OBESITY




use std::io;


fn main() {
    println!("BMI Calculator");
    println!("Enter your weight in kilogram: ");

    let weight = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("Invalid input for weight");
            return;
        }
    };

    println!("Please enter yout height in meters: ");
    let height = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("Invalid input for meter");
            return;
        }
    };

    if height == 0.0 {
        println!("height cannot be zero");
        return;
    }

    let bmi = calculate_bmi(weight,height);
    println!("Your BMI is: {:.2}", bmi);


    let category = classify_bmi(bmi); 
    println!("BMI Category: {}", category);


}



fn get_input_as_f64() -> Option<f64> {

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<f64>() {
        Ok(value) => Some(value),
        Err(_) => None
    }


}


fn calculate_bmi(weight: f64, height: f64) -> f64 {
   weight / height.powi(2) 
}

fn classify_bmi(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "Underweight"
    } else if bmi >= 18.5 && bmi <= 24.9 {
        "Normal Weight"
    } else if bmi >= 18.5 && bmi <= 24.9 {
        "Over Weight" 
    } else {
        "Obesity"
    }

}
