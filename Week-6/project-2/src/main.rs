use std::io;

struct Sibling {
    first_name: String,
    age: u8,
    status: String,
    university: String,
    course_of_study: String,
    offspring: String,
    city: String,
    waec_status: String,
    secondary_school: String,
    class_level: String,
}

fn main() {
    // Ask for the number of siblings
    println!("How many siblings do you have?");
    let mut num_siblings = String::new();
    io::stdin().read_line(&mut num_siblings).unwrap();
    let num_siblings: u8 = num_siblings.trim().parse().unwrap();

    // Create an array to store the details of each sibling
    let mut siblings = vec![Sibling {
        first_name: String::new(),
        age: 0,
        status: String::new(),
        university: String::new(),
        course_of_study: String::new(),
        offspring: String::new(),
        city: String::new(),
        waec_status: String::new(),
        secondary_school: String::new(),
        class_level: String::new(),
    }; num_siblings as usize];

    // Iterate through each sibling and ask for their details
    for i in 0..num_siblings {
        println!("What is the first name of sibling {}?", i + 1);
        let mut first_name = String::new();
        io::stdin().read_line(&mut first_name).unwrap();
        siblings[i as usize].first_name = first_name.trim().to_string();

        println!("What is the age of sibling {}?", i + 1);
        let mut age = String::new();
        io::stdin().read_line(&mut age).unwrap();
        siblings[i as usize].age = age.trim().parse().unwrap();

        if siblings[i as usize].age > 18 {
            println!("Is sibling {} married or single?", i + 1);
            let mut status = String::new();
            io::stdin().read_line(&mut status).unwrap();
            siblings[i as usize].status = status.trim().to_string();

            if siblings[i as usize].status == "single" {
                println!("Is sibling {} a student or a worker?", i + 1);
                let mut student_or_worker = String::new();
                io::stdin().read_line(&mut student_or_worker).unwrap();
                siblings[i as usize].status = student_or_worker.trim().to_string();

                if siblings[i as usize].status == "student" {
                    println!("What university does sibling {} attend?", i + 1);
                    let mut university = String::new();
                    io::stdin().read_line(&mut university).unwrap();
                    siblings[i as usize].university = university.trim().to_string();

                    println!("What course of study is sibling {} pursuing?", i + 1);
                    let mut course_of_study = String::new();
                    io::stdin().read_line(&mut course_of_study).unwrap();
siblings[i as usize].course_of_study = course_of_study.trim().to_string();
}
} else if siblings[i as usize].status == "married" {
println!("Does sibling {} have any offspring?");
let mut offspring = String::new();
io::stdin().read_line(&mut offspring).unwrap();
siblings[i as usize].offspring = offspring.trim().to_string();
}
}
    println!("In which city does sibling {} reside?");
    let mut city = String::new();
    io::stdin().read_line(&mut city).unwrap();
    siblings[i as usize].city = city.trim().to_string();

    println!("What is the WAEC status of sibling {}?");
    let mut waec_status = String::new();
    io::stdin().read_line(&mut waec_status).unwrap();
    siblings[i as usize].waec_status = waec_status.trim().to_string();

    println!("Which secondary school did sibling {} attend?");
    let mut secondary_school = String::new();
    io::stdin().read_line(&mut secondary_school).unwrap();
    siblings[i as usize].secondary_school = secondary_school.trim().to_string();

    println!("What class level is sibling {} in?");
    let mut class_level = String::new();
    io::stdin().read_line(&mut class_level).unwrap();
    siblings[i as usize].class_level = class_level.trim().to_string();
}
