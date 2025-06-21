use std::io;

fn main() {
    println!("Student Report Card Generator");
    println!("-----------------------------");

    // Input student name
    println!("Enter student name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");
    let name = name.trim();

    // Input total marks
    println!("Enter total marks:");
    let mut marks_input = String::new();
    io::stdin().read_line(&mut marks_input).expect("Failed to read marks");
    let total_marks: f32 = marks_input.trim().parse().unwrap_or(0.0);

    // Input number of subjects
    println!("Enter number of subjects:");
    let mut subjects_input = String::new();
    io::stdin().read_line(&mut subjects_input).expect("Failed to read subjects");
    let subjects_count: u32 = subjects_input.trim().parse().unwrap_or(1);

    // Calculate average
    let average = total_marks / subjects_count as f32;

    // Determine grade
    let grade = if average >= 90.0 {
        "A"
    } else if average >= 75.0 {
        "B"
    } else if average >= 60.0 {
        "C"
    } else {
        "D"
    };

    // Print report
    println!("\n=== REPORT CARD ===");
    println!("Name: {}", name);
    println!("Total Marks: {}", total_marks);
    println!("Subjects: {}", subjects_count);
    println!("Average: {:.2}", average);
    println!("Grade: {}", grade);
    println!("==================");
}