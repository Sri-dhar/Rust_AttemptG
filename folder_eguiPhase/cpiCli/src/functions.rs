mod semdata;

pub fn getdata(sem_id :i32){
    if sem_id == 1 {
        let semester = semdata::semester_i;
        println!("Enter your grade for ");
        for (subject, (credits, name)) in semester.sem_info.iter() {
            println!("{}: ", name);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let grade: f64 = input.trim().parse().unwrap();
            let grade_points = grade * *credits as f64;
            // Assuming curCPI is defined and mutable
            curCPI += grade_points;
        }
    }
}