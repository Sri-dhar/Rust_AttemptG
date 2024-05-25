mod semdata;

fn main() {
    // Get all semesters from semdata.rs
    let semesters = semdata::Semester::new();

    // Input from the user
    println!("Info: ");
    println!("Enter 7.1 for Semester VII-I, 7.2 for Semester VII-II, 7.3 for Semester VII-III");
    println!(
        "Enter 8.1 for Semester VIII-I, 8.2 for Semester VIII-II, 8.3 for Semester VIII-III: "
    );
    println!("\nEnter the semester ID: ");
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");

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

    //code below is to print information about a given semester
    /*
    match semester_info {
        Some(semester) => {
            println!("Semester Information for Semester {}", semester.sem_id);
            for (course_code, &(credits, course_name)) in semester.sem_info.iter() {
                println!(
                    "Code: {:<10} Name: {:<20} Credits: {:>3}",
                    course_code, course_name, credits
                );
            }
        }
        None => println!("Semester with ID {} not found!", semester_id),
    }

     */

    // Get grades from the user
    let grades = match semester_info {
        Some(semester) => semester.get_grades_from_user(),
        None => {
            println!("Grades not available for Semester with ID {}", semester_id);
            return;
        }
    };

    println!("\nGrades: ");
    for (course_code, grade) in grades.iter() {
        println!("Course: {:<10} Grade: {}", course_code, grade);
    }
}
