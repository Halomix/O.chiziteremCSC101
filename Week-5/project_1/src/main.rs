use std::io;

fn main() {
    
    println!("\nInput value of a");
    let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Character null");
    let a:f32 = a.trim().parse().expect("Character not Valid");

    
    println!("\ninput value of b");
    let mut b = String::new();
        io::stdin().read_line(&mut b).expect("Character null");
    let b:f32 = b.trim().parse().expect("Charcter not valid");

    
    println!("\nInput constant c");
    let mut c = String::new();
        io::stdin().read_line(&mut c).expect("Character null");
    let c:f32 = c.trim().parse().expect("Character not Valid");

    let d = f32 ::powf(b,2.0);
    let e = d - (4.0 * a * c);
    let f = e.sqrt();
    let r1 = (b + f) / (2.0 * a);
    let r2 = (b - f) / (2.0 * a);

    println!("Your roots are {} and {}.", r1, r2);
}
