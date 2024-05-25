// mod semdata;
// mod functions;

// fn main() 
// {
//     let mut curSem = 0;
//     let mut curCPI = 0.0;

//     //assuming the user is in sem 1
//     let mut what_to_calc = 0;
//     // 1 for CPI, 2 for SPI
//     //take input for what to calc
//     println!("Enter 1 for CPI, 2 for SPI");
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     what_to_calc = input.trim().parse().unwrap();
    
//     if what_to_calc == 2 {
//         println!("You have decided to calculate SPI");
//         println!("Enter the current semester");
//         input = String::new();
//         std::io::stdin().read_line(&mut input).unwrap();
//         curSem = input.trim().parse().unwrap();


//         println!("Enter your grades from a scale of 0 to 10");
//         if curSem == 1
//         {
//             getdata(1);
//         }


//     }
// }


use std::collections::HashMap;
mod semdata;

fn main() {
    // Get all semesters from semdata.rs
    let semesters = semdata::Semester::new();

    // Input from the user
    println!("Enter the semester ID (e.g., 1.0, 2.0, etc.): ");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read input");

    let semester_id: f32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    // Find the semester by ID
    let semester_info = semesters.iter().find(|s| s.sem_id == semester_id);

    // Handle semester existence
    match semester_info {
        Some(semester) => {
            println!("Semester Information for Semester {}", semester.sem_id);
            for (course_code, &(credits, course_name)) in semester.sem_info.iter() {
                println!("Course Code: {}\t Course Name: {} \t Credits: {}", course_code, course_name, credits);
            }
        }
        None => println!("Semester with ID {} not found!", semester_id),
    }
}
