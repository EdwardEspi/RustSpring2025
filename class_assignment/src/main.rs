// Define the Student struct
struct Student {
    major: String,
}

// First-order function to assign a major to a student
fn assign_major(student: &mut Student, major_declared: String) {
    student.major = major_declared;
}

// Higher-order function to update majors for a list of students
fn update_majors(collection: &mut [Student], behavior: fn(&mut Student, String), major: String) {
    for student in collection.iter_mut() {
        behavior(student, major.clone());
    }
}

fn main() {
    // Create a vector of students
    let mut students = vec![
        Student {
            major: String::from("Undeclared"),
        },
        Student {
            major: String::from("Undeclared"),
        },
        Student {
            major: String::from("Undeclared"),
        },
    ];
    // Print initial majors and updated majors
    println!("Before updating majors:");
    for (i, student) in students.iter().enumerate() {
        println!("Student {}: {}", i + 1, student.major);
    }
    
    update_majors(&mut students, assign_major, String::from("Computer Science"));

    println!("\nAfter updating majors:");
    for (i, student) in students.iter().enumerate() {
        println!("Student {}: {}", i + 1, student.major);
    }
}