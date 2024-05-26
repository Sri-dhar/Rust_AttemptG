mod semdata;

fn get_grade_of_student(sem_no: f32) -> Vec<f32> {
    let semesters = semdata::get_semesters(sem_no).unwrap();
    let mut grades = Vec::new(); 
    
    for i in 0..semesters.course_name.len() {
        println!("Enter the grade of {} ({}) with Credit {}", semesters.course_name[i], semesters.course_code[i], semesters.course_credit[i]);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let grade: f32 = input.trim().parse().expect("Invalid grade");
        grades.push(grade);
    }
    
    grades 
}

fn calculate_spi_n(sem_no: f32,grades: Vec<f32>) -> f32 {
    let semesters = semdata::get_semesters(sem_no).unwrap();
    let mut spi = 0.0;
    let total_credit = semesters.total_credit;
    for i in 0..semesters.course_name.len() {
        spi += grades[i] * semesters.course_credit[i];
        // total_credit += semesters.course_credit[i];
    }
    spi / total_credit
}

fn calculate_spi(){
    println!("Enter the semester number you want to calculate SPI of: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let main_sem_no: f32 = input.trim().parse().expect("Invalid semester number");

    // ACCESSING THE SEMDATA MODULE
    // let a = semdata::get_semesters(1.0).unwrap();
    // println!("{:?}", a.course_code);
    // println!("{:?}", a.course_name);
    // println!("{:?}", a.course_credit);

    let grades_recieved_by_student = get_grade_of_student(main_sem_no);
    
    // PRINTING GRADES OF STUDENT
    // for i in 0..grades_recieved_by_student.len() {
    //     println!("Grade of {}({}) is {}", 
    //     semdata::get_semesters(main_sem_no).unwrap().course_code[i],
    //     semdata::get_semesters(main_sem_no).unwrap().course_name[i],
    //     grades_recieved_by_student[i]);
    // }

    let spi = calculate_spi_n(main_sem_no, grades_recieved_by_student);
    println!("SPI of Semester {} is {:.3}", main_sem_no, spi);
}

fn main() {

    println!("What do you want to Calculate? (SPI/CPI): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    let input = input.to_uppercase();
    
    if input == "CPI"{
        println!("Calculating CPI");
        // calculate_cpi();
    }
    else if input == "SPI"{
        println!("Calculating SPI");
        calculate_spi();
    }
    else{
        println!("Invalid Input");
        println!("Exiting the Calculator");
        return;
    }

}
