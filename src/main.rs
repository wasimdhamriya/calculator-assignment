extern crate calculator;
use calculator::calculator as cal;
use std::io;
fn main()
{
    println!("Enter the function you wish to perform");
    loop
    {
    println!("1- Addition\n2- Subtraction\n3- Multiplication\n4- Division\n5- Exponents\n6- Quit\nYour choice");

let mut input = String::new();
io::stdin().read_line(&mut input)
.expect("Failed to read line");
let input: i32 = match input.trim().parse(){
            Ok(input) => input,
            Err(_) => continue
        };
if input == 1
{
    let int = input_val();
    let calcu = cal::cal::calculator(int.0, int.1);
    calcu.addition();
}
    
else if input == 2
{
    let int = input_val();
    let calcu = cal::cal::calculator(int.0, int.1);
    calcu.subtraction();
}
else if input == 3
{
    let int = input_val();
    let calcu = cal::cal::calculator(int.0, int.1);
    calcu.multiplication();
}
else if input == 4
{
    let int = input_val();
    let calcu = cal::cal::calculator(int.0, int.1);
    calcu.division();
}
else if input == 5
{
    let int = input_val();
    let calcu = cal::cal::calculator(int.0, int.1);
    calcu.exponents();
}
else if input == 6
{
println!("Bye, now.");
break;
}
else 
{
println!("Error - Wrong Input");
}

    }
}
pub fn input_val()->(i32,i32)
{
println!("\nNote - If you input 0, the program will close\n");
println!("Enter First Number");
let mut input1 = String::new();
io::stdin().read_line(&mut input1)
.expect("Failed to read line");
let input1: i32 = input1.trim().parse().unwrap();
println!("Enter Second Number");
let mut input2 = String::new();
io::stdin().read_line(&mut input2)
.expect("Failed to read line");
let input2: i32 = input2.trim().parse().unwrap();
(input1,input2)
}