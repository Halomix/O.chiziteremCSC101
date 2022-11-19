use std::io;

fn main() {

    println!("
        Menu                         Amount
        Poundo Yam & Edinkaiko Soup   ₦3,200
        Fried Rice & Chicken        ₦3,000

        Amala & Ewedu Soup          ₦2,500
        Eba & Egusi Soup            ₦2,000

        White Rice & Stew           ₦2,500");

     let mut p = String::new();
     let mut f= String::new();
     let mut a= String::new();
     let mut e = String::new();
     let mut w = String::new();

    println!("How many portions Yam & Edinkaiko soup you dey chop");
        io::stdin()
        .read_line(&mut p)
        .expect("Character null");
    let p:f32 = p.trim().parse().expect("Character not valid");
        
    println!("How many portions Fried Rice & Chicken you dey chop");
        io::stdin()
        .read_line(&mut f)
        .expect("Character null");
    let f:f32 = f.trim().parse().expect("character not valid");
        
    println!("How many portions Amala & Ewedu Soup you dey chop ");
        io::stdin()
        .read_line(&mut a)
        .expect("Character null");
    let a:f32 = a.trim().parse().expect("Character is not valid");
        
    println!("How many portions Eba & Egusi Soup you dey chop");
        io::stdin()
        .read_line(&mut e)
        .expect("Character null");
    let e:f32 = e.trim().parse().expect("Character is not valid");
        
    println!("How many portions White Rice & Stew would you dey chop");
        io::stdin()
        .read_line(&mut w)
        .expect("Character null");
    let w:f32 = w.trim().parse().expect("Character null");

    let t:f32 = (p * 3200.0) + (f * 3000.0) + (a * 2500.0) + (e * 2000.0) + (w * 2500.0);
    println!("Your total price is ₦{}", t);

    
        let d = t * (0.05);
        let amount = t - d;

        if t > 10000.0{
        println!("You dey buy beta food you get discount of 5%
            Your new total is {}", amount);
         }
         else {
            println!("Thank you for your patronage.");
            
         }
}
