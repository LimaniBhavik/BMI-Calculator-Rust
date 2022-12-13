// Formula: BMI Calculation - Body Mass Index (BMI) Calculator can be used to calculate BMI values based on their height and weight. BMI is a fairly reliable indicator of body fatness for most people.

// BMI = (weight) / (height * height)

// BMI table for Adults - WHO
// Severe Thinness	< 16
// Moderate Thinness	16 - 17
// Mild Thinness	17 - 18.5
// Normal	18.5 - 25
// Overweight	25 - 30
// Obese Class I	30 - 35
// Obese Class II	35 - 40
// Obese Class III	> 40

fn main() {
    println!("\nBMI Calculator!\n");
    println!("\nBMI table for Adults by WHO\n
Severe Thinness	< 16
Moderate Thinness	16 - 17
Mild Thinness	17 - 18.5
Normal	18.5 - 25
Overweight	25 - 30
Obese Class I	30 - 35
Obese Class II	35 - 40
Obese Class III	> 40
");
    // get weight from the user
    let mut line = String::new();
    println!("Enter your Weight in KG:");

    // read input line string and store it into line
    std::io::stdin().read_line(&mut line).unwrap();

    // convert line to integer
    let weight_input: f64 = line.trim().parse().unwrap();
    //println!("Your Weight is {}\n",weight_input);

    // get height from the user
    let mut line = String::new();
    println!("Please Enter your Height in CM:");

    // read input line string and store it into line
    std::io::stdin().read_line(&mut line).unwrap();

    // convert line to integer
    let height_input: f64 = line.trim().parse().unwrap();
    //println!("Your Height is {}\n",height_input);

    // do calculation based on the user input
    // BMI = (weight) / (height * height)
    //let bmi_calculation = weight_input / (height_input*height_input);
    let bmi: f64 = weight_input / ((height_input * height_input) / 10000.00);
    // show result to the user
    if bmi < 16.00 {
        println!("Severe Thinness {}", bmi);
    } else if bmi > 16.00 && bmi < 17.00 {
        println!("Moderate Thinness {}", bmi);
    } else if bmi > 17.00 && bmi < 18.5 {
        println!("Mild Thinness {}", bmi);
    } else if bmi > 18.00 && bmi < 25.00 {
        println!("Normal {}", bmi);
    } else if bmi > 25.00 && bmi < 30.00 {
        println!("Overweight {}", bmi);
    } else if bmi > 30.00 && bmi < 35.00 {
        println!("Obese Class I {}", bmi);
    } else if bmi > 35.00 && bmi < 40.00 {
        println!("Obese Class II {}", bmi);
    } else if bmi > 40.00 {
        println!("Obese Class III {}", bmi);
    } else {
        println!("Please enter valid input {}", bmi);
    }
    //println!("Your BMI Calculation is {}\n",bmi);
}
