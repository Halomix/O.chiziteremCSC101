fn main() {
    let name = vec!["Sam", "Sally", "Greg", "Ade", "Mark","June","Ife"];
    
    let age = vec![16,17,18,19,20,21,22,23];

    println!("\nAge allocation:\n");

    for i in 0..age.len()
    {
        print!("{} is {} years old\n",name[i], age[i]);

    }
}
