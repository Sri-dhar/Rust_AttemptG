
fn main() {
    let grade_map: HashMap<&str, f64> = [
        ("AA", 10.0),
        ("AB", 9.0),
        ("BB", 8.0),
        ("BC", 7.0),
        ("CC", 6.0),
        ("CD", 5.0),
        ("DD", 4.0),
        ("F", 0.0),
    ]
    .iter()
    .cloned()
    .collect();

    let semester_i = Semester {
        sem_id: 1.0,
        sem_info: [
            ("MA101", (8, "Mathematics I")),
            ("CS101", (6, "Computer Programming")),
            ("CS110", (3, "Computer Programming Lab")),
            ("EC101", (8, "Digital Design")),
            ("EC110", (3, "Digital Design Lab")),
            ("EC102", (8, "Electrical Circuit Analysis")),
            ("HS101", (4, "English")),
            ("GE101", (6, "Induction Program")),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let semester_ii = Semester {
        sem_id: 2.0,
        sem_info: [
            ("MA102", (8, "Mathematics II")),
            ("CS103", (8, "Data Structures")),
            ("CS111", (3, "Data Structures Lab")),
            ("CS104", (8, "Computer Organization")),
            ("EC103", (8, "Basic Electronic Circuits")),
            ("EC111", (3, "Basic Electronics Lab")),
            ("HSXXX", (6, "HSS Elective")),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    // Similarly, create instances for other semesters
    let semester_iii = Semester {
        sem_id: 3.0,
        sem_info: [
            ("MA203", (6, "Mathematics III")),
            ("MA205", (6, "Discrete Mathematics")),
            ("CS201", (6, "Algorithms")),
            ("CS210", (3, "Algorithm Lab")),
            ("CS202", (7, "IT Workshop I")),
            ("SC201", (6, "Physics I")),
            ("HSXXX", (6, "HSS Elective")),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let semester_iv = Semester {
        sem_id: 4.0,
        sem_info: [
            ("CS205", (6, "Optimization Techniques")),
            ("CS231", (6, "Operating Systems")),
            ("CS232", (3, "Operating Systems Lab")),
            ("CS235", (6, "Artificial Intelligence")),
            ("CS236", (3, "Artificial Intelligence Lab")),
            ("CS240", (6, "Database Management Systems")),
            ("CS241", (4, "DBMS Lab")),
            ("SC202", (6, "Chemistry")),
            ("HSXXX", (6, "HSS Elective")),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let semester_v = Semester {
        sem_id: 5.0,
        sem_info: [
            ("CS301", (6, "Theory of Computation")),
            ("CS352", (6, "Computer Networks")),
            ("CS353", (4, "Computer Networks Lab")),
            ("CS306", (6, "Machine Learning")),
            ("CS360", (3, "Machine Learning Lab")),
            ("CS351", (7, "IT Workshop III : Cloud Computing")),
            ("SC301", (6, "Biology")),
            ("HSXXX", (6, "HSS Elective")),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let semester_vi = Semester {
        sem_id: 6.0,
        sem_info: [
            ("CS330", (6, "Software Engineering")),
            ("CS331", (3, "Software Engineering Lab")),
            ("CS320", (6, "Compilers")),
            ("CS321", (3, "Compilers Lab")),
            ("CS361", (6, "Computer Security")),
            ("SC302", (6, "Physics II")),
            ("CS300", (6, "Project-I / Elective - I")),
            ("HSXXX", (6, "HSS Elective")),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let semester_vii_i = Semester {
        sem_id: 7.1,
        sem_info: [
            ("CS401", (6, "Data Analytics")),
            ("CS4XX", (6, "Open Elective")),
            ("CS4XX", (6, "Elective I")),
            ("CS4XX", (6, "Elective II")),
            ("HSXXX", (6, "HSS Elective")),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let semester_vii_ii = Semester {
        sem_id: 7.2,
        sem_info: [
            ("CS401", (6, "Data Analytics")),
            ("CS4XX", (6, "Open Elective")),
            ("CS4XX", (6, "Elective I")),
            ("CS400", (6, "Project II")),
            ("HSXXX", (6, "HSS Elective")),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let semester_viii_i = Semester {
        sem_id: 8.1,
        sem_info: [
            ("CS4XX", (6, "Elective")),
            ("CS4XX", (6, "Elective")),
            ("CS4XX", (6, "Elective")),
            ("CS4XX", (6, "Elective")),
            ("HS4XX", (6, "Elective (HSS)")),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let semester_viii_ii = Semester {
        sem_id: 8.2,
        sem_info: [
            ("CS4XX", (6, "Elective")),
            ("HS4XX", (6, "Elective (HSS)")),
            ("CS410", (18, "Project III")),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    let semester_viii_iii = Semester {
        sem_id: 8.3,
        sem_info: [
            ("CS4XX", (6, "Elective")),
            ("HS4XX", (6, "Elective (HSS)")),
            ("CS411", (18, "Internship")),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    println!("Semester I: {:?}", semester_i.sem_info);
    println!("Semester II: {:?}", semester_ii.sem_info);
    // Print other semesters...
}
