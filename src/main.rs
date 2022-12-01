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
  println!("BMI Calculator!");
  // get weight from the user
  let mut line  = String::new();
  println!("Enter your Weight in KG:");
  
  // read input line string and store it into line
  std::io::stdin().read_line(&mut line).unwrap();
  
  // convert line to integer
  let weight_input :f64 = line.trim().parse().unwrap();
  //println!("Your Weight is {}\n",weight_input);

  // get height from the user
  let mut line  = String::new();
  println!("Please Enter your Height in CM:");

  // read input line string and store it into line
  std::io::stdin().read_line(&mut line).unwrap();
  
  // convert line to integer
  let height_input :f64 = line.trim().parse().unwrap();
  //println!("Your Height is {}\n",height_input);
  
  // do calculation based on the user input 
  // BMI = (weight) / (height * height)
  //let bmi_calculation = weight_input / (height_input*height_input);
  let bmi:f64 = weight_input / ((height_input * height_input)/ 10000.00);
  // show result to the user
  if bmi < 18.6 {
      println!("Law {}", bmi);
   } else if bmi > 18.6 && bmi < 24.9 {
      println!("Normal {}", bmi);
   } else {
      println!("Over Weight {}", bmi) ;
   }
  //println!("Your BMI Calculation is {}\n",bmi);
}

