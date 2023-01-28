use std::fs::File;
use std::io::prelude::*;

fn main() {
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", "300"), 
        ("Adams Aliyu", "ECO10110101", "Economics", "100"), 
        ("Shanla Bolade", "CSC10328828", "Computer Science", "200"), 
        ("Adekunle Gold", "EEE11020202", "Electrical Engineering", "200"), 
        ("Blanca Edemoh", "MEE10202001", "Mechanical Engineering", "100")
    ];

    let mut sims_file = File::create("PAU_SIMS.txt").expect("Failed to create file");
    let header = "S/N  | Student Name      |   Matric. Number  | Department             | Level\n";
    sims_file.write_all(header.as_bytes()).expect("Failed to write header");
    sims_file.write_all("_____________________________________________________________________________\n".as_bytes()).expect("Failed to write separator");

    for (i, student) in students.iter().enumerate() {
        let line = format!("{:<3} | {:<20} | {:<16} | {:<25} | {:<5}\n", i+1, student.0, student.1, student.2, student.3);
        sims_file.write_all(line.as_bytes()).expect("Failed to write student data");
    }

    let mut sims = File::open("PAU_SIMS.txt").expect("Failed to open file");
    let mut sims_content = String::new();
    sims.read_to_string(&mut sims_content).expect("Failed to read file");
    println!("{}", sims_content);
}




