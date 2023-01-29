use std::io::Read;
use std::io;
  fn criteria() {
 println!("\nWELCOME TO GLOBACOM
 	what is your role\n
 	A. Administrator
 	B. Manager
 	C. Employee
 	D. Customer
 	E. Vendor");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1:char = input1.trim().parse().expect("Not a valid integer");
    println!("{}",input1 );




if  input1 == 'A'
    {
    let mut file = std::fs::File::open("globacom.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents );
}
else if input1 == 'B'
{
    let mut file4 = std::fs::File::open("globacom.sql").unwrap();
    let mut contents4 = String::new();
    file4.read_to_string(&mut contents4).unwrap();
    print!("{}",contents4 );
}

 else if input1 == 'C'
 {  
 	let mut file1 = std::fs::File::open("staff_tb.sql").unwrap();
 	let mut contents1 = String::new();
 	file1.read_to_string(&mut contents1).unwrap();
    print!("{}",contents1 );
}
 else if input1 == 'D'
{
	let mut file2 = std::fs::File::open("customer_tb.sql").unwrap();
	let mut contents2 = String::new();
	file2.read_to_string(&mut contents2).unwrap();
	print!("{}",contents2 );
}
else if input1 == 'E'
{
     let mut file3 = std::fs::File::open("dataplan_tb.sql").unwrap();
 	let mut contents3 = String::new();
 	file3.read_to_string(&mut contents3).unwrap();
    print!("{}",contents3);
}
}  

 
    

     fn main() {
     	criteria()
    	
    }

  	

