use crate::semdata;

pub fn get_grade_of_student(sem_no: f32) -> Vec<f32> {
    let semesters: semdata::Semester = semdata::get_semesters(sem_no).unwrap();
    let mut grades: Vec<f32> = Vec::new(); 
    
    for i in 0..semesters.course_name.len() {
        println!("Enter the grade of {} ({}) with Credit {}", semesters.course_name[i], semesters.course_code[i], semesters.course_credit[i]);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let grade: f32 = input.trim().parse().expect("Invalid grade");
        grades.push(grade);
    }
    
    grades 
}

pub fn calculate_spi_n(sem_no: f32, grades: Vec<f32>) -> f32 {
    let semesters = semdata::get_semesters(sem_no).unwrap();
    let mut spi = 0.0;
    let total_credit = semesters.total_credit;
    for i in 0..semesters.course_name.len() {
        spi += grades[i] * semesters.course_credit[i];
    }
    spi / total_credit
}

pub fn calculate_spi(){
    println!("Enter the semester number you want to calculate SPI of: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let main_sem_no: f32 = input.trim().parse().expect("Invalid semester number");

    let grades_recieved_by_student = get_grade_of_student(main_sem_no);
    let spi = calculate_spi_n(main_sem_no, grades_recieved_by_student);
    println!("SPI of Semester {} is {:.3}", main_sem_no, spi);
}

pub fn calculate_cpi_option1(x: f32)
{
    println!("Calculating CPI of sem {} based on CPI of sem {}\n", x+1.0, x);
    println!("Enter the value of CPI of sem {} : ", x);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let cpi = input.trim().parse::<f32>().unwrap();

    let grade_for_sem_x_plus_1 = get_grade_of_student(x+1.0);
    let spi = calculate_spi_n(x+1.0, grade_for_sem_x_plus_1);

    let semester_x = semdata::get_semesters(x).unwrap();
    let semester_x_plus_1 = semdata::get_semesters(x+1.0).unwrap();

    let credit_of_sem_x_plus_1 :f32 = semester_x_plus_1.total_credit;
    let cum_sum_of_credit_till_sem_x :f32 = semester_x.total_credit_till_sem;

    let cpi_of_sem_x_plus_1 = (cpi * cum_sum_of_credit_till_sem_x + spi * credit_of_sem_x_plus_1) / (cum_sum_of_credit_till_sem_x + credit_of_sem_x_plus_1);
    println!("SPI of sem {} is {:.3} and ", x+1.0, spi);
    println!("CPI of sem {} is {:.3}", x+1.0, cpi_of_sem_x_plus_1);
}

pub fn calculate_cpi_option2(x: f32)
{
    println!("Calculating CPI of sem {} based on SPI of sem {} and CPI till sem {}\n", x, x-1.0, x-1.0);
    println!("Enter the value of SPI of sem {} : ", x);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let spi = input.trim().parse::<f32>().unwrap();

    println!("Enter the value of CPI till sem {} : ", x-1.0);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let cpi_till_sem_x_minus_1 = input.trim().parse::<f32>().unwrap();

    let semester_x = semdata::get_semesters(x).unwrap();
    let semester_x_minus_1 = semdata::get_semesters(x-1.0).unwrap();

    let credit_of_sem_x :f32 = semester_x.total_credit;
    let cum_sum_of_credit_till_sem_x_minus_1 :f32 = semester_x_minus_1.total_credit_till_sem;

    let cpi_of_sem_x = (cpi_till_sem_x_minus_1 * cum_sum_of_credit_till_sem_x_minus_1 + spi * credit_of_sem_x) / (cum_sum_of_credit_till_sem_x_minus_1 + credit_of_sem_x);
    println!("SPI of sem {} is {:.3} and ", x, spi);
    println!("CPI of sem {} is {:.3}", x, cpi_of_sem_x);
}

pub fn calculate_cpi()
{
    println!("Select An Option : ");
    println!("1. You have CPI of sem x and want to calculate CPI of sem x+1");
    println!("2. You have SPI of sem x and CPI till sem x-1 and want to calculate CPI of sem x");
    println!();
    println!("Enter the option: ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let option = input.parse::<i32>().unwrap();
    
    println!();
    println!("Enter the value of x (Semester): ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let x = input.parse::<f32>().unwrap();

    if option == 1{
        calculate_cpi_option1(x);
    }
    else if option == 2{
        calculate_cpi_option2(x);
    }
    else{
        println!("Invalid Input");
        println!("Exiting the Calculator");
        return;
    }

}
