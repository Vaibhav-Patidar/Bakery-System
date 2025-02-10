#[allow(unused_imports)]
use std::io;

fn main() {
     
    println!("Welcome to my shop!!!");

    println!("Enter the amount u want - ");
    let mut amount = String::new();
    
    let mut input = String::new();    //create empty variables for input

    io::stdin()               //used to get a user input
        .read_line(&mut amount)  //mut used as variable values can be changed
        .expect("Failed to read line");
    println!("Enter the sweet u want - ");
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
 

    let amount:i32 = amount.trim().parse().expect("Please enter a valid number");
    //above line - trimming the input value so that it doesn't go to another line and converting it to integer
    let input = input.trim();
    

  second(amount, input); //calling the function and giving it the appropriate inputs

  let x = num();  
    if x > amount {
        println!(", Thank you for your purchase");
    }
    else if x < amount {
        println!(" but Amount of {} left is {}", input, x);
        //yes or no input 
       let mut answer = String::new();
       println!("will u take {} {} or you don't want any?,", x, input);
       println!("if you want then type 1, if u don't type 2");

    io::stdin()
     .read_line(&mut answer)
     .expect("Failed to read line");

     let answer:i32 = answer.trim().parse().expect("Please enter a valid number");

   //if else nesting
    if answer == 1 {
       print!(", Thank you for the purchase, visit again");
    } else if answer == 2 {
       print!("OK, Visit Again");
    } else {
       print!("please enter a valid number");
    }
    }
    else {
        println!(", Thank you for your purchase");
    }

  println!(", Visit Again 😁😁");

  println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}



fn second(_value: i32, _amt: &str){    //setting up parameters for the inputs we need
    print!("so u want {} {}", _value, _amt);
    
}


fn num() -> i32{    //inventory of sweets used this just to understand the phenomenon of parameters
  5
}

