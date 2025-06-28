use std::io;
 
fn main() {
    println!("Option #1: From Celsius to Farenheit"); 
    println!("Option #2: From Farenheit to Celsius"); 

    let choice = loop {
        println!("Please select an option: "); 

        let mut choice = String::new(); 

        io::stdin()
            .read_line(&mut choice).expect("Failed to read line"); 

        // ignoring a non-number input  
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Choice must be either 1 or 2");
                continue},
        }; 

        if choice == 1 || choice == 2 {
            break choice; 
        } else {
            println!("Choice must be either 1 or 2"); 
        }
    }; 

    let temp = loop {
        println!("Please enter a degree: "); 

        let mut temp = String::new(); 

        io::stdin()
            .read_line(&mut temp).expect("Failed to read line"); 

        // ignoring a non-number input  
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Degree must be a number"); 
                continue}, 
        }; 

        break temp; 
    }; 

    let temp = if choice == 1 {
        c_to_f(temp) 
    } else {
        f_to_c(temp) 
    }; 

    println!("Conversion result: {temp:.2}"); 
}

fn c_to_f(temp: f64) -> f64 {
    temp * 1.8 + 32. 
}

fn f_to_c(temp: f64) -> f64 {
    (temp - 32.) / 1.8
}