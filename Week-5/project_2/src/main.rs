use std::io;

fn main() 
{
    println!("\nYearly salary for Employees");

    
    let mut exp = String::new();
    println!("\nExperienced worker,yes? input A. Inxperienced input B.");
    io::stdin().read_line(&mut exp).expect("Character null");
    let exp:char = exp.trim().parse().expect("Character not valid");



    let mut age = String::new();
    println!("What's Your age");
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Not a valid integer");

    if exp == 'B'
    {
        let  u:i32 = 100000 * 12;
        println!("Your annual salary is ₦{}.", u);
    }
    else if (age >= 40) && (exp == 'A')
    {
        let u:i32 = 1500000 * 12;
        println!("Your annual salary is ₦{}.", u);
    }
    else if (age >= 30) && (age < 40) && (exp == 'A' )
    {
        let u:i32 = 1480000 * 12;
        println!("Your annual salary is ₦{}.", u);
    }
    else if (age < 28) && (exp == 'A')
    {
        let u:i32 = 1300000* 12;
        println!("Your annual salary is ₦{}.", u);
    }
    else
    {
        println!("Chracter null");
    }
}
