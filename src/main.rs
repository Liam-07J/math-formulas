// A way to solve math formulas Such as:
// Pythagorean theorem
// Qua&dratic equation
// Factorials

use std::{io};
fn main() {
    loop {
        // Get input from user
        println!("What formula do you want to solve?1. Pythagorean theorem 2. Quadratic equation 3. Factorials");
        let mut formula = String::new();
        io::stdin().read_line(&mut formula).expect("Failed to read line");
        print!("{}",formula);
        // Convert input to integer
        let formula: i32 = formula.trim().parse().expect("Please type a number!");
        // excute the formula
        match formula {
            1 => pytheagorean(),
            2 => quadratic(),
            3 => factorial(),
            _ => println!("Please type a number between 1 and 4!"),
        }
    }
}

fn pytheagorean(){
    // TODO Check if C > a + b
    println!("Do you know what the value of C is? 1.Yes or 2.No");
    let mut c_is_known = String::new();
    io::stdin().read_line(&mut c_is_known).expect("Failed to read line");
    //convert input to integer
    let c_is_known: i32 = c_is_known.trim().parse().expect("Please type a number!");

    if c_is_known == 2{
        println!("What is the value of A?");
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Failed to read line");
        println!("What is the value of B?");
        let mut b = String::new();
        io::stdin().read_line(&mut b).expect("Failed to read line");
        print!("A = {} B = {}", a, b);
        // convert input to float
        let a: f32 = a.trim().parse().expect("An error has occured");
        let b: f32 = b.trim().parse().expect("An error has occured");
        let answer :f32 = (a*a+b*b).sqrt();
        println!("The answer is {}", answer);
    }
    else if c_is_known == 1{println!("What is the value of A?");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    println!("What is the value of C?");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read line");
    print!("A = {} B = {}", a, c);
    // convert input to float
    let a: f32 = a.trim().parse().expect("An error has occured");
    let c: f32 = c.trim().parse().expect("An error has occured");
    let answer :f32 = (c*c-a*a).sqrt();
    println!("The answer is {}", answer);}
    else{
        println!("Please type yes or no!");
        pytheagorean()
    }
}
fn quadratic(){
    // Find the value of A,B,C
    println!("What is the value of A?");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    println!("What is the value of B?");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read line");
    println!("What is the value of C?");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read line");
     // convert input to float
     let a: f32 = a.trim().parse().expect("An error has occured");
     let b: f32 = b.trim().parse().expect("An error has occured");
     let c: f32 = c.trim().parse().expect("An error has occured");
    print!("{}x^2 + {}x + {} = 0 \n", a, b, c);
    // Find the value of x
    let x1:f32 = (-b + (b * b - 4.0 * a * c).sqrt())/(2.0 * a);
    let x2:f32 = (-b - (b * b - 4.0 * a * c).sqrt())/(2.0 * a);
    //if x is equal to NaN
    if x1.is_nan(){
        println!("This is not allowed! Please try again!");
        quadratic();
    }
    else{
        println!("One Value of x is {} another is {}",x1,x2);
    }
}
fn factorial(){
    // Get input from user
    println!("What is the value of n?");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    // if n has a decimal in it
    if n.contains("."){
        println!("Please type a whole number!");
        factorial();
    }
    else{
    // convert input to integer
    let n: i32 = n.trim().parse().expect("Please type a number!");
    // calculate factorial
    let mut factorial = 1.0;
    for i in 1..(n as u32 + 1){
        factorial = factorial * i as f32;
    }
    println!("The factorial of {} is {}", n, factorial);
    start_again();
    }
}
fn start_again(){
    loop{
        println!("Do you want to solve another formula? 1.Yes or 2.No");
        let mut start_again = String::new();
        io::stdin().read_line(&mut start_again).expect("Failed to read line");
        //convert input to integer
        let start_again: i32 = start_again.trim().parse().expect("Please type a number!");
        if start_again == 1{
            main();
        }
        else if start_again == 2{
            println!("Thank you for using this program!");
            return;
        }
        else{
            println!("Please type yes or no!");
        }
    }
}